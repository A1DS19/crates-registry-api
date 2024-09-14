use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name=crate::schema::user_role)]
pub struct NewUserRole {
    user_id: i32,
    role_id: i32,
}
