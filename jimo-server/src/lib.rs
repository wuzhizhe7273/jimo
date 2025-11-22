mod api;
pub mod banner;
mod router;
mod service;
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server,
};
use std::{convert::Infallible, net::SocketAddr, pin, time::Duration};
use anyhow::Context;
use tower::Service;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::{error, info};
use jimo_conf::{JimoConfMgr, JimoCtxConf};
use crate::router::router;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    addr: String,
    port: u16,
}

impl jimo_conf::JomoConfItem for ServerConfig {
    fn name() -> &'static str {
        "server"
    }
}

impl ServerConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}
pub struct JimoServer {
    ctx:jimo_ctx::JimoCtx
}
impl JimoServer {
    pub fn new(config: JimoConfMgr,ctx:jimo_ctx::JimoCtx) -> Self {
        ctx.insert_conf_mgr(config);
        Self {ctx}
    }
    pub async fn run(self) -> anyhow::Result<()> {
       let addr=self.ctx.config::<ServerConfig>().await?.context("server config not found")?.addr();
        info!("server listen on http://{}", addr);
        let ctx = jimo_ctx::JimoCtx::new();
        let app = router(ctx)
            .layer(
                TraceLayer::new_for_http(), // .make_span_with(|req:&Request<_>|{
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
        let mut make_service = app.into_make_service_with_connect_info::<SocketAddr>();
        let graceful = hyper_util::server::graceful::GracefulShutdown::new();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let server =
            hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new());
        let mut ctrl_c = std::pin::pin!(tokio::signal::ctrl_c());
        loop {
            tokio::select! {
              conn=listener.accept()=>{
                  let (socket,remote_addr)=match conn{
                      Ok(conn)=>conn,
                      Err(e)=>{
                          error!("accept connection error: {:?}", e);
                          continue;
                      }
                  };
                  info!("incoming connection accepted: {}", remote_addr);
                  let socket=TokioIo::new(socket);
                  let tower_service=unwrap_infallible(make_service.call(remote_addr).await);
                  let hyper_service=hyper::service::service_fn(move |req:hyper::Request<Incoming>|{
                        let mut value=tower_service.clone();
                        async move{
                           if req.uri().path()=="/p/"{
                                unreachable!()
                            }else{
                                value.call(req).await
                            }
                        }
                  });
                    let conn = server.serve_connection_with_upgrades(socket, hyper_service);
                  let conn = graceful.watch(conn.into_owned());
                  tokio::spawn(async move {
                        if let Err(err) = conn.await  {
                              error!("serve connection error: {:?}", err);
                          }
                          error!("connection dropped: {}",remote_addr);

                  });
              },
              _ =ctrl_c.as_mut()=>{
                drop(listener);
                info!("shutdown signal received, {} connections pending", graceful.count());
                break;
              }
            }
        }
        tokio::select! {
            _=graceful.shutdown()=>{
                info!("graceful shutdown completed");
            },
            _=tokio::time::sleep(Duration::from_secs(10))=>{
                info!("shutdown timeout, force shutdown");
            }
        }
        Ok(())
    }
}
fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => match err {},
    }
}
