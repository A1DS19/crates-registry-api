use crate::{models::rustacean::Rustacean, schema::rustacean};
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::generic_repository::GenericRepository;

pub struct RustaceanRepository {}

impl GenericRepository<Rustacean> for RustaceanRepository {
    type NewEntity = crate::dtos::new_rustacean::NewRustacean;

    async fn get_all(connection: &mut AsyncPgConnection) -> QueryResult<Vec<Rustacean>> {
        rustacean::table.load(connection).await
    }

    async fn get_by_id(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<Rustacean> {
        rustacean::table.find(id).get_result(connection).await
    }

    async fn add(
        entity: &Self::NewEntity,
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(rustacean::table)
            .values(entity)
            .execute(connection)
            .await
    }

    async fn update(
        entity: &Rustacean,
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustacean::table.find(entity.id))
            .set((
                rustacean::name.eq(&entity.name),
                rustacean::email.eq(&entity.email),
            ))
            .get_result(connection)
            .await
    }

    async fn delete(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::delete(rustacean::table.find(id))
            .execute(connection)
            .await
    }
}
