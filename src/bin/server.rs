use std::{net::SocketAddr, sync::Arc};

use shiori::build_axum_router;
use shiori_core::ShioriCore;
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    shiori::config::tracing::init();

    let core = ShioriCore::new();
    let app = Arc::new(core.get_app());

    let axum_router = build_axum_router(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Shiori server starting on http://{}", addr);

    axum::serve(listener, axum_router).await.unwrap();
}
