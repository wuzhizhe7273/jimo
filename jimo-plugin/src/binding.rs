wasmtime::component::bindgen!({
    world:"plugin",
    with:{
        "wasi:http@0.2.6":wasmtime_wasi_http::bindings::http
    }
});
