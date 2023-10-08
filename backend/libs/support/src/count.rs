use error::{Error, Result};
use sea_orm::{
    prelude::{EntityTrait, PaginatorTrait},
    DatabaseConnection,
};

///
/// Function that takes count of entities in table
/// 
pub async fn count<T>(db: &DatabaseConnection) -> Result<u64>
where
    T: EntityTrait + Send + Sync,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find();
    PaginatorTrait::count(query, db).await.map_err(Error::from)
}
