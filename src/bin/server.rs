use std::sync::Arc;

use shiori::build_handler;
use shiori_core::ShioriCore;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let core = ShioriCore::new();
    let app = Arc::new(core.get_app());

    let axum_router = build_handler(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, axum_router).await.unwrap();
}
