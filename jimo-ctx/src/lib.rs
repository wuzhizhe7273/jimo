#![feature(trait_alias)]

use std::sync::Arc;
pub use typedmap::TypedMapKey as JimoCtxItemKey;
use typedmap::clone::SyncCloneBounds;
#[derive(Debug, Clone)]
pub struct JimoCtx {
    inner: Arc<typedmap::TypedDashMap<(), SyncCloneBounds, SyncCloneBounds>>,
}
impl JimoCtx {
    pub fn default() -> Self {
        Self {
            inner: Arc::new(typedmap::TypedDashMap::new_with_bounds()),
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
}

impl JimoCtx {
    pub fn insert<K>(&self, key: K, value: K::Value)
    where
        K: JimoCtxItemKey + Send + Sync + Clone + 'static,
        K::Value: Send + Sync + Clone,
    {
        self.inner.insert(key, value);
    }
    pub fn get<K>(&self, key: &K) -> Option<K::Value>
    where
        K: JimoCtxItemKey + Send + Sync + Clone + 'static,
        K::Value: Send + Sync + Clone,
    {
        self.inner.get(key).map(|v| v.value().clone())
    }
}
