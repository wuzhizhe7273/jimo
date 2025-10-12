#![feature(trait_alias)]
use typedmap::{clone::SyncCloneBounds, impl_dyn_trait_wrapper};
pub trait JimoCtxItem{}
pub use typedmap::TypedMapKey as JimoCtxItemKey;
impl_dyn_trait_wrapper!(DynJimoCtxItem, JimoCtxItem);

#[derive(Debug,Clone)]
pub struct JimoCtx {
    inner: typedmap::TypedDashMap<(),SyncCloneBounds,SyncCloneBounds>,
}
impl JimoCtx {
    pub fn default()->Self{
        Self{inner:typedmap::TypedDashMap::new_with_bounds()}
    }
    pub fn new()->Self{
        Self::default()
    }
}

impl JimoCtx {
    pub fn insert<K>(&self, key: K, value: K::Value)
    where
        K: typedmap::TypedMapKey + Send + Sync + Clone+'static,
        K::Value: JimoCtxItem + Send + Sync+Clone,
    {
        self.inner.insert(key, value);
    }
    pub fn get<K>(&self, key: &K) -> Option<K::Value>
    where
        K: typedmap::TypedMapKey + Send + Sync + Clone+'static,
        K::Value: JimoCtxItem + Send + Sync+Clone,
    {
        self.inner.get(key).map(|v|v.value().clone())
    }
}