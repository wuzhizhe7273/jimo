mod router;
mod api;
mod service;
pub mod banner;
use std::time::Duration;
use jimo_conf::{JimoConfMgr, JimoCtxConfKey};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::router::router;

#[derive(serde::Serialize,serde::Deserialize)]
pub struct ServerConfig{
    addr:String,
    port:u16
}

impl jimo_conf::JomoConfItem for ServerConfig {
    fn name()->&'static str {
        "server"
    }
}

impl ServerConfig {
    pub fn addr(&self)->String{
        format!("{}:{}",self.addr,self.port)
    }
}
pub struct JimoServer{
    config:JimoConfMgr,
}
impl JimoServer {
    pub fn new(config:JimoConfMgr)->Self{
        Self { config }
    }
    pub async fn run(self)->anyhow::Result<()>{
        let addr=self.config.get::<ServerConfig>().await?;
        info!("server listen on http://{}",addr.addr());
        let ctx=jimo_ctx::JimoCtx::new();
        ctx.insert(JimoCtxConfKey, self.config);
        let app=router(ctx)
            .layer(
                TraceLayer::new_for_http()
                // .make_span_with(|req:&Request<_>|{
                //     let matched_path=req.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
                //     info_span!(
                //         "http-request",
                //         method = ?req.method(),
                //         matched_path,
                //         some_other_field = tracing::field::Empty,
                //     )
                // })
            )
            .layer(TimeoutLayer::new(Duration::from_secs(10)));
        let listener=tokio::net::TcpListener::bind(addr.addr()).await?;
        axum::serve(listener,app).with_graceful_shutdown(shutdown_signal()).await?;
        Ok(())
    }
}

async fn shutdown_signal(){
    let ctrl_c=async{
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(not(unix))]
    let terminate=std::future::pending::<()>();
    tokio::select! {
        _=ctrl_c=>{},
        _=terminate=>{},
    }
}