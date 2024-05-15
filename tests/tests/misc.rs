use serde::Deserialize;

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
