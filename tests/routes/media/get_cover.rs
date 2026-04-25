use axum::body::Bytes;
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_database::models::NewMedia;
use shiori_test::{test_app::TestApp, util::RequestHelper};
use tokio::fs;

#[tokio::test(flavor = "multi_thread")]
async fn get_cover() {
    let (app, _, user) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let tmp_dir = tempfile::tempdir().unwrap();
    let cover_path = tmp_dir.path().join("cover.png");

    fs::write(&cover_path, b"fake cover").await.unwrap();

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: "book.epub",
        extension: "epub",
        library_id: 1,
        cover_path: Some(cover_path.to_str().unwrap()),
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}/cover", m.id);

    let response = user.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"200 OK");

    // Make sure file content is the same
    let body_bytes = Bytes::copy_from_slice(response.body());
    let expected_bytes: Bytes = fs::read(&cover_path).await.unwrap().into();
    assert_eq!(body_bytes, expected_bytes)
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found_no_path() {
    let (app, _, user) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: "book.epub",
        extension: "epub",
        library_id: 1,
        cover_path: None,
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}/cover", m.id);

    let response = user.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"404 Not Found");
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found_no_file() {
    let (app, _, user) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: "book.epub",
        extension: "epub",
        library_id: 1,
        cover_path: Some("/fake/path"),
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}/cover", m.id);

    let response = user.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"404 Not Found");
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found_media() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.get::<()>("/api/v1/media/999/cover").await;
    assert_snapshot!(response.status(), @"404 Not Found");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Not Found\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn unauthorized() {
    let (app, anon) = TestApp::init_empty().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let tmp_dir = tempfile::tempdir().unwrap();
    let cover_path = tmp_dir.path().join("cover.png");

    fs::write(&cover_path, b"fake cover").await.unwrap();

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: "book.epub",
        extension: "epub",
        library_id: 1,
        cover_path: Some(cover_path.to_str().unwrap()),
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}/cover", m.id);

    let response = anon.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}
