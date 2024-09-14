use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

pub trait GenericRepository<Entity> {
    type NewEntity;

    async fn get_all(connection: &mut AsyncPgConnection) -> QueryResult<Vec<Entity>>;

    async fn get_by_id(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<Entity>;

    async fn add(
        entity: &Self::NewEntity,
        connection: &mut AsyncPgConnection,
    ) -> QueryResult<usize>;

    async fn update(entity: &Entity, connection: &mut AsyncPgConnection) -> QueryResult<Entity>;

    async fn delete(id: i32, connection: &mut AsyncPgConnection) -> QueryResult<usize>;
}
