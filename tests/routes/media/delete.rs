use diesel::OptionalExtension;
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_database::models::{Media, NewMedia};
use shiori_test::{test_app::TestApp, util::RequestHelper};
use tokio::fs;

#[tokio::test(flavor = "multi_thread")]
async fn returns_media_by_id() {
    let (app, _, user) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    app.new_library("library", "/data/books").await;

    let tmp_dir = tempfile::tempdir().unwrap();
    let book_path = tmp_dir.path().join("book.epub");
    let cover_path = tmp_dir.path().join("cover.png");

    fs::write(&book_path, b"fake book").await.unwrap();
    fs::write(&cover_path, b"fake cover").await.unwrap();

    let new_media = NewMedia {
        name: "book",
        size: 1,
        path: book_path.to_str().unwrap(),
        extension: "epub",
        library_id: 1,
        cover_path: Some(cover_path.to_str().unwrap()),
        koreader_hash: None,
    };

    let m = new_media.insert(&mut conn).await.unwrap();
    let url = format!("/api/v1/media/{}", m.id);

    let response = user.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"204 No Content");

    let response = user.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"404 Not Found");

    assert!(!book_path.exists());
    assert!(!cover_path.exists());

    let m = Media::find(&mut conn, m.id).await.optional().unwrap();
    assert!(m.is_none())
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.delete::<()>("/api/v1/media/999").await;
    assert_snapshot!(response.status(), @"404 Not Found");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Not Found\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn unauthorized() {
    let (app, anon) = TestApp::init_empty().await;
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
    let url = format!("/api/v1/media/{}", m.id);

    let response = anon.delete::<()>(&url).await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}
