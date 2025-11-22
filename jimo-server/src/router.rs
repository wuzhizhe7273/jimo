use std::{net::Incoming, usize};

use axum::{
    Extension, Router,
    extract::{MatchedPath, Request},
    middleware::{self, Next},
    response::Response,
    routing::get,
    serve::IncomingStream,
};
use jimo_ctx::JimoCtx;

pub fn router(ctx: JimoCtx) -> Router {
    Router::new()
        .route("/ping", axum::routing::get(|| async { "hello world" }))
        .route("/test", get(test_handler))
        .layer(Extension(ctx))
}
#[axum::debug_handler]
async fn test_handler(req: axum::extract::Request) -> Response {
    Response::new("test".into())
}
