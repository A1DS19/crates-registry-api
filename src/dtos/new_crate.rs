use diesel::prelude::Insertable;
use serde::Deserialize;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::crate_)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
