use crate::models::rustacean::Rustacean;
use crate::repositories::rustacean_repository::RustaceanRepository;
use crate::DbConn;
use crate::{
    dtos::new_rustacean::NewRustacean, repositories::generic_repository::GenericRepository,
};
use rocket::serde::json::Json;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_db_pools::Connection;

#[get("/")]
pub async fn index(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    match RustaceanRepository::get_all(&mut db).await {
        Ok(rustaceans) => Ok(json!(rustaceans)),

        Err(e) => {
            println!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[get("/<id>")]
pub async fn get_by_id(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    match RustaceanRepository::get_by_id(id, &mut db).await {
        Ok(rustacean) => Ok(json!(rustacean)),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Rustacean not found")));
            }

            println!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[post("/", format = "json", data = "<new_rustacean>")]
pub async fn add(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    match RustaceanRepository::add(&new_rustacean, &mut db).await {
        Ok(_) => Ok(Custom(Status::Created, json!("Rustacean added"))),

        Err(e) => {
            println!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[put("/", format = "json", data = "<rustacean>")]
pub async fn update(
    mut db: Connection<DbConn>,
    rustacean: Json<Rustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    match RustaceanRepository::update(&rustacean, &mut db).await {
        Ok(updated_rustacean) => Ok(Custom(Status::Ok, json!(updated_rustacean))),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Rustacean not found")));
            }

            println!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}

#[delete("/<id>")]
pub async fn delete(mut db: Connection<DbConn>, id: i32) -> Result<Custom<Value>, Custom<Value>> {
    match RustaceanRepository::delete(id, &mut db).await {
        Ok(_) => Ok(Custom(Status::Ok, json!("Rustacean deleted"))),

        Err(e) => {
            if e == diesel::result::Error::NotFound {
                return Err(Custom(Status::NotFound, json!("Rustacean not found")));
            }

            println!("An error occurred: {:?}", e);
            Err(Custom(
                Status::InternalServerError,
                json!("An error occurred"),
            ))
        }
    }
}
