use actix_web::http::StatusCode;
use actix_web::test;
use serde::Deserialize;

use crate::common::{create_app, TestApp};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub total_item: i64,
    pub total_page: i64,
    pub page_size: i64,

    pub current: i64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Place {
    pub id: i32,
    pub name: String,
}

#[actix_web::test]
async fn test_list_places() {
    let app = create_app().await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/metadata/places"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let events: Vec<Place> = test::read_body_json(resp).await;
    assert!(!events.is_empty());
}
