use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::role)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}
