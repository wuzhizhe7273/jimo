use std::sync::Arc;

use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use wasmtime::{Store, component::Linker};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{
    WasiHttpCtx, WasiHttpView,
    bindings::http::types::Scheme,
    body::{HyperIncomingBody, HyperOutgoingBody},
};

use crate::binding::Plugin;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct JimoPluginKey(pub String);
impl jimo_ctx::JimoCtxItemKey for JimoPluginKey {
    type Value = Arc<JimoPlugin>;
}

struct JimoPluginStore {
    id: Option<String>,
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for JimoPluginStore {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}
impl WasiHttpView for JimoPluginStore {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
#[derive(Clone)]
pub struct JimoPlugin {
    engine: wasmtime::Engine,
    pub id: String,
    plugin: Arc<Plugin>,
}

// plugin load
impl JimoPlugin {
    pub fn load_from_url(url: &url::Url, engine: wasmtime::Engine) -> anyhow::Result<Self> {
        let component = match url.scheme() {
            "file" => wasmtime::component::Component::from_file(&engine, url.path())?,
            _ => {
                panic!("unsupported scheme")
            }
        };
        let mut store = Self::new_store(&engine, None);
        let linker = Linker::new(&engine);
        let plugin = Plugin::instantiate(&mut store, &component, &linker).unwrap();
        let name = plugin.jimo_plugin_info().call_name(&mut store)?;
        Ok(JimoPlugin {
            id: name,
            engine,
            plugin: Arc::new(plugin),
        })
    }
    pub fn load_from_binary(bytes: impl AsRef<[u8]>, engine: wasmtime::Engine) -> anyhow::Result<Self> {
        let component = wasmtime::component::Component::new(&engine, bytes.as_ref())?;
        let mut store = Self::new_store(&engine, None);
        let linker = Linker::new(&engine);
        let plugin = Plugin::instantiate(&mut store, &component, &linker).unwrap();
        let name = plugin.jimo_plugin_info().call_name(&mut store)?;
        Ok(JimoPlugin {
            id: name,
            engine,
            plugin: Arc::new(plugin),
        })
    }
    fn new_store(engine: &wasmtime::Engine, id: Option<String>) -> Store<JimoPluginStore> {
        Store::new(
            engine,
            JimoPluginStore {
                id,
                wasi: WasiCtx::builder().inherit_stdio().build(),
                http: WasiHttpCtx::new(),
                table: ResourceTable::new(),
            },
        )
    }
}
// plugin lifecycle management
impl JimoPlugin {
    fn init(&self, ctx: jimo_ctx::JimoCtx) -> anyhow::Result<()> {
        let store = Self::new_store(&self.engine, Some(self.id.clone()));
        self.plugin
            .jimo_plugin_management_hanlder()
            .call_init(store)
            .unwrap()
            .unwrap();
        Ok(())
    }
    fn start(&self, ctx: jimo_ctx::JimoCtx) -> anyhow::Result<()> {
        let store = Self::new_store(&self.engine, Some(self.id.clone()));
        self.plugin
            .jimo_plugin_management_hanlder()
            .call_start(store)
            .unwrap()
            .unwrap();
        Ok(())
    }
    fn stop(&self, ctx: jimo_ctx::JimoCtx) -> anyhow::Result<()> {
        let store = Self::new_store(&self.engine, Some(self.id.clone()));
        self.plugin
            .jimo_plugin_management_hanlder()
            .call_stop(store)
            .unwrap()
            .unwrap();
        Ok(())
    }
    fn cleanup(&self, ctx: jimo_ctx::JimoCtx) -> anyhow::Result<()> {
        let store = Self::new_store(&self.engine, Some(self.id.clone()));
        self.plugin
            .jimo_plugin_management_hanlder()
            .call_cleanup(store)
            .unwrap()
            .unwrap();
        Ok(())
    }
}

impl JimoPlugin {
    async fn handle_request(
        &self,
        req: hyper::Request<Incoming>,
    ) -> anyhow::Result<hyper::Response<HyperOutgoingBody>> {
        let mut store = Self::new_store(&self.engine, Some(self.id.clone()));
        let (tx, cx) = tokio::sync::oneshot::channel();
        let req = store.data_mut().new_incoming_request(Scheme::Http, req)?;
        let out = store.data_mut().new_response_outparam(tx)?;
        self.plugin
            .jimo_plugin_request_handler()
            .call_handle(store, req, out)
            .unwrap();
        match cx.await {
            Ok(Ok((resp))) => Ok(resp),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(e.into()),
        }
    }
}
