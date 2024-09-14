use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::role)]
pub struct NewRole {
    code: String,
    name: String,
}
