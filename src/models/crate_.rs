use chrono::NaiveDateTime;
use diesel::prelude::Queryable;

#[derive(Debug, Queryable)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}
