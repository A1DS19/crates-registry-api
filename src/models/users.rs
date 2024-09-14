use chrono::NaiveDateTime;

use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    created_at: NaiveDateTime,
}
