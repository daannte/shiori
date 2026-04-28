use insta::{assert_json_snapshot, assert_snapshot};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn post() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "name": "cool token"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"200 OK");

    let json = response.json();
    assert!(json.get("plaintoken").is_some());
    assert_json_snapshot!(json, {
        ".created_at" => "[datetime]",
        ".last_used_at" => "[datetime]",
        ".key_id" => "[key_id]",
        ".plaintoken" => "[plaintoken]"
    });

    let body = serde_json::json!({
        "expires_at": "2026-12-31T00:00:00Z",
        "name": "cool token"
    });

    let response_b = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response_b.status(), @"200 OK");

    let json_b = response_b.json();
    assert!(json_b.get("expires_at").is_some());

    assert_json_snapshot!(json_b, {
        ".created_at" => "[datetime]",
        ".expires_at" => "[datetime]",
        ".last_used_at" => "[datetime]",
        ".key_id" => "[key_id]",
        ".plaintoken" => "[plaintoken]"
    });

    assert_ne!(json.get("plaintoken"), json_b.get("plaintoken"));
}

#[tokio::test(flavor = "multi_thread")]
async fn post_anon() {
    let (_, anon) = TestApp::init_empty().await;

    let body = serde_json::json!({
        "expires_at": null,
        "name": "cool token"
    });

    let response = anon.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn cant_create_token_with_token() {
    let (_, _, _, user) = TestApp::init_with_token().await;

    let body = serde_json::json!({
        "expires_at": null,
        "name": "cool token"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"400 Bad Request");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"API token cannot be used to create another API token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_date_in_body() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "expires_at": "2026-12-31",
        "name": "cool token"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: expires_at: premature end of input at line 1 column 26""#);

    let body = serde_json::json!({
        "expires_at": "2024-12-31T00:00:00Z",
        "name": "cool token"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"400 Bad Request");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"token expiration cannot be in the past\"}""#);

    let body = serde_json::json!({
        "expires_at": "2024-15-31T00:00:00Z",
        "name": "cool token"
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: expires_at: input is out of range at line 1 column 36""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn invalid_name_in_body() {
    let (_, _, user) = TestApp::init_with_user().await;

    let body = serde_json::json!({
        "expires_at": "2026-12-31T00:00:00Z",
        "name": ""
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"400 Bad Request");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"name must have a value\"}""#);

    let body = serde_json::json!({
        "expires_at": "2026-12-31T00:00:00Z",
    });

    let response = user.post::<()>("/api/v1/tokens", body.to_string()).await;
    assert_snapshot!(response.status(), @"422 Unprocessable Entity");
    assert_json_snapshot!(response.text(), @r#""Failed to deserialize the JSON body into the target type: missing field `name` at line 1 column 37""#);
}
