use jimo_conf::{JimoConfMgr, JimoFigmentStorage};
use jimo_server::{JimoServer, banner};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use jimo_ctx::JimoCtx;

fn main() {
    banner::print();
    tracing_subscriber::registry().with(fmt::layer()).init();
    let config = JimoFigmentStorage::builder()
        .path("jimo.toml")
        .build()
        .unwrap();
    let config = JimoConfMgr::new(config);
    let ctx=JimoCtx::new();
    let server = JimoServer::new(config,ctx);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            server.run().await.unwrap();
        });
}
