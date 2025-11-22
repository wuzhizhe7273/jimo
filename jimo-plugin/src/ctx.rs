use std::ffi::OsStr;
use std::sync::Arc;
use jimo_conf::JimoCtxConf;
use crate::config::PluginConfig;
use crate::plugin::{JimoPlugin, JimoPluginKey};
use crate::WasmtimeEngineKey;

pub trait JimoCtxPlugin{
    async fn load_plugins(&self)->anyhow::Result<()>;
}
impl JimoCtxPlugin for jimo_ctx::JimoCtx{
    async fn load_plugins(&self) -> anyhow::Result<()> {
        let engine=wasmtime::Engine::default();
        self.insert(WasmtimeEngineKey,engine);
        let config=self.config::<PluginConfig>().await?.expect("plugin path not found");
       let plugins=walkdir::WalkDir::new(config.path())
           .into_iter()
           .filter_entry(|e|e.file_type().is_file()&&e.path().extension()==Some(OsStr::new("wasm")))
           .filter_map(|e|e.ok().map(|v|v.into_path())).collect::<Vec<_>>();
        for p in plugins{
            let engine=self.get(&WasmtimeEngineKey).expect("wasmtime engine not found");
            let url=url::Url::from_file_path(p).expect("wasm url from path");
            let plugin=JimoPlugin::load_from_url(&url,engine)?;
            self.insert(JimoPluginKey(plugin.id.clone()),Arc::new(plugin));
        }
        Ok(())
    }
}