use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::users)]
pub struct NewUser {
    username: String,
    password: String,
}
