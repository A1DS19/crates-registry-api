use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::user_role)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
