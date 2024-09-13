use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rustacean)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
