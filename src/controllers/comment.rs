use actix_web::{get, post, web, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::Result;
use crate::orm::comment::{Comment, CommentDisplay, NewComment};
use crate::orm::utils::RunQueryDsl;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct NewCommentForm {
    #[validate(length(min = 1))]
    pub content: String,
}

#[get("/{id}/comment")]
async fn get_comments(state: web::Data<AppState>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let comments: Vec<CommentDisplay> = Comment::by_event_id_as_display(id)
        .select(CommentDisplay::as_select())
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(comments))
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
