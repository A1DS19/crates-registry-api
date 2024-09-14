use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    id: i32,
    code: String,
    name: String,
    created_at: NaiveDateTime,
}
