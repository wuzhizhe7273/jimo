mod commiter;
pub use commiter::EventCommiter;
mod fetcher;
pub use fetcher::{AggregateFetcher, EventFetcher};

pub trait EventExecutor<'s> {
    type Ret;
    type Error: Send + Sync + 'static;
    type Store;
    async fn execute(&'s mut self, store: &'s Self::Store) -> Result<Self::Ret, Self::Error>;
}
