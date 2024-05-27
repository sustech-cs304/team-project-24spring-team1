use actix_web::{get, post, web, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::Result;
use crate::orm::comment::{Comment, CommentDisplay, NewComment};
use crate::orm::schema::comments;
use crate::orm::utils::RunQueryDsl;
use crate::utils::page::Page;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct CommentQuery {
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
struct NewCommentForm {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Serialize)]
struct CommentsResponse {
    pub page: Page,
    pub comments: Vec<CommentDisplay>,
}

#[get("/{id}/comment")]
async fn get_comments(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<CommentQuery>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let total_item = Comment::by_event_id(id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let comments: Vec<CommentDisplay> = Comment::by_event_id_as_display(id)
        .select(CommentDisplay::as_select())
        .order(comments::id.asc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(CommentsResponse { page, comments }))
}

#[post("/{id}/comment")]
async fn post_comment(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<NewCommentForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let form = form.into_inner();
    form.validate()?;

    let new_comment = NewComment {
        account_id: auth.account_id,
        event_id: id,
        content: &form.content,
    };

    new_comment
        .as_insert()
        .execute(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comments).service(post_comment);
}
