use crate::common::{aggregate, projection::ProjectionStore};

use std::future::Future;
pub trait Query<Ret, Store, Error>
where
    Store: ProjectionStore,
    Error: std::error::Error + Send + Sync + 'static,
{
    async fn execute(self, store: &Store) -> Result<Ret, Error>;
}

impl<F, Fut, Ret, Store, Error> Query<Ret, Store, Error> for F
where
    Store: ProjectionStore,
    Error: std::error::Error + Send + Sync + 'static,
    Fut: Future<Output = Result<Ret, Error>>,
    F: FnOnce(&Store) -> Fut,
{
    async fn execute(self, store: &Store) -> Result<Ret, Error> {
        (self)(store).await
    }
}

pub trait QueryById<Id, Ret, Store, Error>: Query<Ret, Store, Error>
where
    Store: ProjectionStore,
    Error: std::error::Error + Send + Sync + 'static,
{
    fn by_id(id: Id) -> Self;
}

pub trait QueryByKey<Key, Ret, Store, Error>: Query<Ret, Store, Error>
where
    Key: aggregate::Key,
    Store: ProjectionStore,
    Error: std::error::Error + Send + Sync + 'static,
{
    fn by_key(key: Key) -> Self;
}
