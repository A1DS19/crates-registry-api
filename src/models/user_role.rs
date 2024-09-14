use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserRole {
    id: i32,
    user_id: i32,
    role_id: i32,
}
