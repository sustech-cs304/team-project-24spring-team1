use actix_web::{delete, get, post, web, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::moment::{Moment, MomentDisplay, NewMoment};
use crate::orm::moment_comment::{MomentComment, MomentCommentDisplay, NewMomentComment};
use crate::orm::schema::{moment_comments, moments};
use crate::orm::utils::RunQueryDsl;
use crate::utils::page::{Page, PaginateQuery};

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct NewMomentForm {
    pub content: String,
}

#[derive(Debug, Serialize)]
struct MomentIdResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
struct ListMomentsResponse {
    pub page: Page,
    pub moments: Vec<MomentDisplay>,
}

#[derive(Debug, Deserialize, Validate)]
struct NewMomentCommentForm {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Serialize)]
struct MomentCommentsResponse {
    pub page: Page,
    pub comments: Vec<MomentCommentDisplay>,
}

#[post("")]
async fn new_moment(
    state: web::Data<AppState>,
    form: web::Json<NewMomentForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let form = form.into_inner();
    form.validate()?;

    let new_moment = NewMoment {
        account_id: auth.account_id,
        content: &form.content,
    };

    let id = new_moment
        .as_insert()
        .returning(moments::id)
        .get_result(&mut state.pool.get().await?)
        .await?;

    debug!("New moment created, id={id}");
    Ok(web::Json(MomentIdResponse { id }))
}

#[get("")]
async fn list_moments(
    state: web::Data<AppState>,
    query: web::Query<PaginateQuery>,
) -> Result<impl Responder> {
    let query = query.into_inner();

    let total_item = Moment::all_display()
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let moments = Moment::all_display()
        .order(moments::id.desc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ListMomentsResponse { page, moments }))
}

#[get("/{id}")]
async fn get_moment(state: web::Data<AppState>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let moment: MomentDisplay = Moment::find_display(id)
        .get_result(&mut state.pool.get().await?)
        .await?;
    Ok(web::Json(moment))
}

#[delete("/{id}")]
async fn delete_moment(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let organizer_id = Moment::find(id)
        .select(moments::account_id)
        .get_result::<i32>(&mut state.pool.get().await?)
        .await?;
    if organizer_id != auth.account_id {
        return Err(Error::Unauthorized(
            "You can only delete moments you created".into(),
        ));
    }

    let affected = Moment::update(id)
        .set(moments::is_deleted.eq(true))
        .execute(&mut state.pool.get().await?)
        .await?;
    // The moment is guaranteed to exist
    assert_eq!(affected, 1);

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

#[get("/{id}/comment")]
async fn get_moment_comments(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<PaginateQuery>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let total_item = MomentComment::by_moment_id(id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let moment_comments: Vec<MomentCommentDisplay> = MomentComment::by_moment_id_as_display(id)
        .select(MomentCommentDisplay::as_select())
        .order(moment_comments::id.asc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(MomentCommentsResponse {
        page,
        comments: moment_comments,
    }))
}

#[post("/{id}/comment")]
async fn post_moment_comment(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<NewMomentCommentForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let form = form.into_inner();
    form.validate()?;

    let new_moment_comment = NewMomentComment {
        account_id: auth.account_id,
        moment_id: id,
        content: &form.content,
    };

    new_moment_comment
        .as_insert()
        .execute(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(new_moment)
        .service(list_moments)
        .service(get_moment)
        .service(delete_moment)
        .service(get_moment_comments)
        .service(post_moment_comment);
}
