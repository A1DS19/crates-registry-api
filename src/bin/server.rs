use c8s::routes::crate_routes;
use c8s::routes::rustacean_routes;
use c8s::routes::DbConn;
use rocket::launch;
use rocket::routes;
use rocket_db_pools::Database;

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
