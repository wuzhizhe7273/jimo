use std::sync::Arc;

use dashmap::DashSet;

#[derive(Clone)]
pub(crate) struct JimoPlugRouter {
    map: Arc<DashSet<String>>,
}

impl JimoPlugRouter {
    pub(crate) fn register(&self, plugin: String) -> bool {
        self.map.insert(plugin)
    }
    pub(crate) fn unregister(&self, plugin: String) -> Option<String> {
        self.map.remove(&plugin)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct JimoCtxPlugRouterKey;

impl jimo_ctx::JimoCtxItemKey for JimoCtxPlugRouterKey {
    type Value = JimoPlugRouter;
}
pub trait JimoPluginRouterCtx {
    fn plugin_register_router(&self, plugin: String) -> bool;
    fn plugin_unregister_router(&self, plugin: String) -> Option<String>;
}
impl JimoPluginRouterCtx for jimo_ctx::JimoCtx {
    fn plugin_register_router(&self, plugin: String) -> bool {
        let router = self
            .get(&JimoCtxPlugRouterKey)
            .expect("JimoPluginRouter not found");
        router.register(plugin)
    }
    fn plugin_unregister_router(&self, plugin: String) -> Option<String> {
        let router = self
            .get(&JimoCtxPlugRouterKey)
            .expect("JimoPluginRouter not found");
        router.unregister(plugin)
    }
}
