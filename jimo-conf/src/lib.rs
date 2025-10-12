mod error;
mod figment;
pub use error::{JimoConfError,JimoConfStorageError};
pub use figment::{JimoFigmentStorage};
use std::sync::Arc;

pub trait JomoConfItem:serde::Serialize+for<'a> serde::Deserialize<'a> {
    fn name()->&'static str;
}

#[async_trait::async_trait]
pub trait JimoConfStorage:Send+Sync{
    async fn get(&self,name:&str)->Result<Option<serde_json::Value>,JimoConfStorageError>;
    async fn set(&self,name:&str,value:&serde_json::Value)->Result<(),JimoConfStorageError>;
}
#[derive(Clone)]
pub struct JimoConfMgr{
    storage:Arc<dyn JimoConfStorage>
}

impl JimoConfMgr {
    pub fn new<S:JimoConfStorage+'static>(storage:S)->Self{
        Self { storage: Arc::new(storage) }
    }
    pub async fn get<C:JomoConfItem>(&self)->Result<C,JimoConfError>{
        let v=self.storage.get(C::name()).await?.ok_or(JimoConfError::item_not_found(C::name()))?;
        let c=serde_json::from_value(v)?;
        Ok(c)
    }
    pub async fn set<C:JomoConfItem>(&self,c:&C)->Result<(),JimoConfError>{
        let v=serde_json::to_value(c)?;
        self.storage.set(C::name(),&v).await?;
        Ok(())
    }
}

#[derive(Debug,Clone,PartialEq, Eq,Hash)]
pub struct JimoCtxConfKey;
impl jimo_ctx::JimoCtxItemKey for JimoCtxConfKey {
    type Value=JimoConfMgr;
}
impl jimo_ctx::JimoCtxItem for  JimoConfMgr {
    
}