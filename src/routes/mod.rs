use rocket_db_pools::Database;

pub mod crate_routes;
pub mod rustacean_routes;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
