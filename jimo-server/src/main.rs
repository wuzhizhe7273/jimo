use jimo_conf::{JimoConfMgr, JimoFigmentStorage};
use jimo_server::{banner, JimoServer};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    banner::print();
    tracing_subscriber::registry().with(fmt::layer()).init();
    let config = JimoFigmentStorage::builder()
        .path("jimo.toml")
        .build()
        .unwrap();
    let config = JimoConfMgr::new(config);
    let server = JimoServer::new(config);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            server.run().await.unwrap();
        });
}
