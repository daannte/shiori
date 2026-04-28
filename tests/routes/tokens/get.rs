use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn get() {
    let (_, _, user, _) = TestApp::init_with_token().await;

    let response = user.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 1);
    assert_json_snapshot!(response.json(), {
        "[].created_at" => "[datetime]",
        "[].expires_at" => "[datetime]",
        "[].last_used_at" => "[datetime]",
        "[].key_id" => "[key_id]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn get_anon() {
    let (_, anon, _, _) = TestApp::init_with_token().await;

    let response = anon.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn no_tokens() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn user_cannot_access_other_users_tokens() {
    let (app, _, user_a, _) = TestApp::init_with_token().await;

    let user_b = app.new_user("user_b", false).await;

    let response = user_a.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 1);

    let response = user_b.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn ignore_expired_tokens() {
    let (_, _, user, _) = TestApp::init_with_token().await;

    user.new_token(
        "test token",
        Some(chrono::Utc::now() - chrono::Duration::seconds(5)),
    )
    .await;

    let response = user.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn returns_tokens_in_descending_creation_order() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "expires_at": null,
        "name": "older"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"200 OK");

    let body = serde_json::json!({
        "expires_at": null,
        "name": "newer"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"200 OK");

    let response = user.get::<()>("/api/v1/tokens").await;
    assert_snapshot!(response.status(), @"200 OK");

    let tokens: Vec<serde_json::Value> = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(tokens.len(), 2);

    assert_eq!(tokens[0]["name"], "newer");
    assert_eq!(tokens[1]["name"], "older");
}
