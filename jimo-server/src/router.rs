use axum::{Extension, Router};
use jimo_ctx::JimoCtx;

pub fn router(ctx:JimoCtx)->Router{
    Router::new().route("/ping", axum::routing::get(||async{"hello world"})).layer(Extension(ctx))
}