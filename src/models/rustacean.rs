use chrono::NaiveDateTime;
use diesel::prelude::Queryable;

#[derive(Debug, Queryable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
