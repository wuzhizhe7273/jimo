use crate::{JimoConfMgr, JomoConfItem};
use crate::config::JimoCtxConfKey;
use crate::error::JimoConfError;

pub trait JimoCtxConf {
    fn insert_conf_mgr(&self, config: JimoConfMgr);
    fn get_conf_mgr(&self) -> Option<JimoConfMgr>;
    async fn config<C: JomoConfItem>(&self) -> Result<Option<C>, JimoConfError>;
}

impl JimoCtxConf for jimo_ctx::JimoCtx {
    fn insert_conf_mgr(&self, config: JimoConfMgr) {
        self.insert(JimoCtxConfKey, config);
    }
    fn get_conf_mgr(&self) -> Option<JimoConfMgr> {
        self.get(&JimoCtxConfKey)
    }
    async fn config<C: JomoConfItem>(&self) -> Result<Option<C>, JimoConfError> {
        let mgr = self.get_conf_mgr().ok_or(JimoConfError::MgrNotFound)?;
        Ok(mgr.get::<C>().await?)
    }
}