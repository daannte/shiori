use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn me() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.get::<()>("/api/v1/auth/me").await;

    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn anon_me() {
    let (_, anon) = TestApp::init_empty().await;

    let response = anon.get::<()>("/api/v1/auth/me").await;

    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.json(), @r#"
    {
      "error": "Unauthorized"
    }
    "#);
}
