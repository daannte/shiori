use std::sync::Arc;

use insta::assert_snapshot;
use shiori::BaseOpenApi;
use shiori_test::test_app::TestApp;

#[tokio::test(flavor = "multi_thread")]
async fn test_open_api_snapshot() {
    let (app, _) = TestApp::init_empty().await;
    let (_, newest) = BaseOpenApi::build(Arc::new(app.as_inner().clone()));
    assert_snapshot!(newest.to_pretty_json().unwrap())
}
