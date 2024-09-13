use chrono::NaiveDateTime;
use diesel::prelude::Queryable;

#[derive(Debug, Queryable)]
pub struct Crate {
    id: i32,
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}
