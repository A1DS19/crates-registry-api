use crate::{models::crate_::Crate, schema::crate_};
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::generic_repository::GenericRepository;

pub struct CrateRepository {}

impl GenericRepository<Crate> for CrateRepository {
    type NewEntity = crate::dtos::new_crate::NewCrate;

    async fn get_all(connection: &mut AsyncPgConnection) -> QueryResult<Vec<Crate>> {
        crate_::table.load(connection).await
    }

    async fn get_by_id(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<Crate> {
        crate_::table.find(id).get_result(connection).await
    }

    async fn add(
        entity: &Self::NewEntity,
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<usize> {
        diesel::insert_into(crate_::table)
            .values(entity)
            .execute(connection)
            .await
    }

    async fn update(entity: &Crate, connection: &mut AsyncPgConnection) -> QueryResult<Crate> {
        diesel::update(crate_::table.find(entity.id))
            .set((crate_::id.eq(&entity.id),))
            .get_result(connection)
            .await
    }

    async fn delete(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::delete(crate_::table.find(id))
            .execute(connection)
            .await
    }
}
