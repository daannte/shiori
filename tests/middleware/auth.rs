use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_database::{models::ApiToken, schema::api_tokens};
use shiori_test::{mock_users::MockTokenUser, test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn unauthorized_jwt() {
    let (_, anon) = TestApp::init_empty().await;

    let response = anon.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn authorized_jwt() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn unauthorized_token() {
    let (app, _, user, token_user) = TestApp::init_with_token().await;
    let mut conn = app.db_conn().await;

    let token = token_user.as_model();

    ApiToken::delete(&mut conn, token.key_id.clone(), user.as_model())
        .await
        .unwrap();

    let response = token_user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Invalid API token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn unauthorized_expired_token() {
    use chrono::{Duration, Utc};
    use diesel::update;

    let (app, _, _, token_user) = TestApp::init_with_token().await;
    let mut conn = app.db_conn().await;

    let token = token_user.as_model();
    let yesterday = Utc::now() - Duration::days(1);

    update(api_tokens::table)
        .filter(api_tokens::token_hash.eq(token.token_hash.clone()))
        .set(api_tokens::expires_at.eq(yesterday))
        .execute(&mut conn)
        .await
        .unwrap();

    let response = token_user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Invalid API token\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn authorized_token() {
    let (_, _, _, token_user) = TestApp::init_with_token().await;

    let response = token_user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn update_last_used_at() {
    let (app, _, _, token_user) = TestApp::init_with_token().await;
    let mut conn = app.db_conn().await;

    let token = token_user.as_model();

    let _ = token_user.get::<()>("/api/v1/auth/me").await;

    let token_db = ApiToken::find_by_hash(&mut conn, &token.token_hash)
        .await
        .unwrap();

    // `last_used_at` is None by default when using `init_with_token`
    assert!(token_db.last_used_at.is_some());
}

#[tokio::test(flavor = "multi_thread")]
async fn malformed_token() {
    let (app, _) = TestApp::init_empty().await;

    let token_user = MockTokenUser::with_auth_header("shiori_invalid_token".to_string(), app);

    let response = token_user.get::<()>("/api/v1/auth/me").await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Invalid API token\"}""#);
}
