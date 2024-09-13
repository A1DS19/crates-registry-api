use chrono::NaiveDateTime;
use diesel::prelude::Queryable;

#[derive(Debug, Queryable)]
pub struct Rustacean {
    id: i32,
    name: String,
    email: String,
    created_at: NaiveDateTime,
}
