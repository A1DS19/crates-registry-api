mod dtos;
mod models;
mod repositories;
mod routes;
mod schema;

use rocket_db_pools::Database;
use routes::crate_routes;
use routes::rustacean_routes;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            routes![
                crate_routes::index,
                crate_routes::get_by_id,
                crate_routes::add,
                crate_routes::update,
                crate_routes::delete,
                rustacean_routes::index,
                rustacean_routes::get_by_id,
                rustacean_routes::add,
                rustacean_routes::update,
                rustacean_routes::delete,
            ],
        )
        .attach(DbConn::init())
}
