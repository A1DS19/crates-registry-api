use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
