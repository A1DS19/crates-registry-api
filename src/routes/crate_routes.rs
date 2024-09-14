use crate::models::crate_::Crate;
use crate::repositories::crate_repository::CrateRepository;
use crate::routes::DbConn;
use crate::{dtos::new_crate::NewCrate, repositories::generic_repository::GenericRepository};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

#[get("/crate")]
pub async fn index(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    match CrateRepository::get_all(&mut db).await {
        Ok(rustaceans) => Ok(json!(rustaceans)),

        Err(e) => {
            rocket::error!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[get("/crate/<id>")]
pub async fn get_by_id(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    match CrateRepository::get_by_id(id, &mut db).await {
        Ok(rustacean) => Ok(json!(rustacean)),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Crate not found")));
            }

            rocket::error!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[post("/crate", format = "json", data = "<new_crate>")]
pub async fn add(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    match CrateRepository::add(&new_crate, &mut db).await {
        Ok(_) => Ok(Custom(Status::Created, json!("Crate added"))),

        Err(e) => {
            rocket::error!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[put("/crate", format = "json", data = "<rustacean>")]
pub async fn update(
    mut db: Connection<DbConn>,
    rustacean: Json<Crate>,
) -> Result<Custom<Value>, Custom<Value>> {
    match CrateRepository::update(&rustacean, &mut db).await {
        Ok(updated_rustacean) => Ok(Custom(Status::Ok, json!(updated_rustacean))),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Rustacean not found")));
            }

            rocket::error!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[delete("/crate/<id>")]
pub async fn delete(mut db: Connection<DbConn>, id: i32) -> Result<Custom<Value>, Custom<Value>> {
    match CrateRepository::delete(id, &mut db).await {
        Ok(_) => Ok(Custom(Status::Ok, json!("Rustacean deleted"))),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Rustacean not found")));
            }

            rocket::error!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}
