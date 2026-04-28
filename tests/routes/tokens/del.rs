use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn delete() {
    let (_, _, user, token_user) = TestApp::init_with_token().await;

    let url = format!("/api/v1/tokens/{}", token_user.as_model().key_id);

    let response = user.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"204 No Content");
    assert!(response.body().is_empty());

    let response = user.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"404 Not Found");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Not Found\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn token_fail_after_delete() {
    let (_, _, _, token_user) = TestApp::init_with_token().await;

    let url = format!("/api/v1/tokens/{}", token_user.as_model().key_id);

    let response = token_user.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"204 No Content");
    assert!(response.body().is_empty());

    let response = token_user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Invalid API token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn anon_cant_delete() {
    let (_, anon, _, token_user) = TestApp::init_with_token().await;

    let url = format!("/api/v1/tokens/{}", token_user.as_model().key_id);

    let response = anon.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn user_cannot_delete_other_users_tokens() {
    let (app, _, _, token_user) = TestApp::init_with_token().await;

    let user_b = app.new_user("user_b", false).await;

    let url = format!("/api/v1/tokens/{}", token_user.as_model().key_id);

    let response = user_b.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"404 Not Found");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Not Found\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_key_id() {
    let (_, _, user, _) = TestApp::init_with_token().await;

    let response = user.delete::<()>("/api/v1/tokens/invalid-key").await;
    assert_snapshot!(response.status(), @"404 Not Found");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Not Found\"}""#);
}
