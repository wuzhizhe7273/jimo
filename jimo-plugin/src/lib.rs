use std::{any::Any, sync::Arc};

use jimo_ctx::JimoCtx;
use wasmtime::{
    Store,
    component::{Instance, Linker},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView, bindings::http::types::Scheme};

use crate::binding::Plugin;
mod binding;
mod plugin;
mod router;
mod ctx;
mod config;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct WasmtimeEngineKey;
impl jimo_ctx::JimoCtxItemKey for WasmtimeEngineKey {
    type Value = wasmtime::Engine;
}
