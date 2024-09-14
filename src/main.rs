mod dtos;
mod models;
mod repositories;
mod routes;
mod schema;

use rocket_db_pools::Database;
use routes::rustacean_routes::{add, delete, get_by_id, index, update};

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index, get_by_id, add, update, delete])
        .attach(DbConn::init())
}
