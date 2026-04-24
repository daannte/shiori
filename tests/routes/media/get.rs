use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use insta::{assert_json_snapshot, assert_snapshot};
use shiori_database::models::{NewMedia, UpdateReadingProgress};
use shiori_test::{test_app::TestApp, util::RequestHelper};

#[tokio::test(flavor = "multi_thread")]
async fn returns_media_by_id() {
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
    let url = format!("/api/v1/media/{}", m.id);

    let response = user.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"200 OK");
    assert_json_snapshot!(response.json(), {
        ".created_at" => "[datetime]"
    });
}

#[tokio::test(flavor = "multi_thread")]
async fn not_found() {
    let (_, _, user) = TestApp::init_with_user().await;

    let response = user.get::<()>("/api/v1/media/999").await;
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

    let response = anon.get::<()>(&url).await;
    assert_snapshot!(response.status(), @"401 Unauthorized");
    assert_json_snapshot!(response.text(), @r#""{\"error\":\"Unauthorized\"}""#);
}

#[tokio::test(flavor = "multi_thread")]
async fn diff_progress_for_users() {
    let (app, _, user_a) = TestApp::init_with_user().await;
    let mut conn = app.db_conn().await;

    let user_a_model = user_a.as_model();
    let user_b = app.new_user("user_b").await;
    let user_b_model = user_b.as_model();

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

    UpdateReadingProgress {
        user_id: user_a_model.id,
        media_id: m.id,
        device_id: None,
        koreader_progress: None,
        percentage_completed: BigDecimal::from_f32(0.5),
        updated_at: Utc::now(),
        completed: false,
        completed_at: None,
    }
    .upsert(&mut conn)
    .await
    .unwrap();

    UpdateReadingProgress {
        user_id: user_b_model.id,
        media_id: m.id,
        device_id: None,
        koreader_progress: None,
        percentage_completed: BigDecimal::from_f32(1.0),
        updated_at: Utc::now(),
        completed: true,
        completed_at: Some(Utc::now()),
    }
    .upsert(&mut conn)
    .await
    .unwrap();

    let url = format!("/api/v1/media/{}", m.id);

    let res_a = user_a.get::<()>(&url).await;
    let body_a = res_a.json();
    assert_snapshot!(res_a.status(), @"200 OK");
    assert_json_snapshot!(body_a, {
        ".created_at" => "[datetime]",
        ".progress.updated_at" => "[datetime]",
        ".progress.started_at" => "[datetime]",
        ".progress.completed_at" => "[datetime]"
    });

    let res_b = user_b.get::<()>(&url).await;
    let body_b = res_b.json();
    assert_snapshot!(res_b.status(), @"200 OK");
    assert_json_snapshot!(body_b, {
        ".created_at" => "[datetime]",
        ".progress.updated_at" => "[datetime]",
        ".progress.started_at" => "[datetime]",
        ".progress.completed_at" => "[datetime]"
    });

    // Make sure data is different per user
    assert_snapshot!(body_a["progress"]["percentage_completed"], @r#""0.5000""#);
    assert_snapshot!(body_b["progress"]["percentage_completed"], @r#""1.0000""#);
    assert_snapshot!(body_a["progress"]["completed"], @"false");
    assert_snapshot!(body_b["progress"]["completed"], @"true");

    assert_ne!(
        body_a["progress"]["percentage_completed"],
        body_b["progress"]["percentage_completed"]
    );
    assert_ne!(
        body_a["progress"]["completed"],
        body_b["progress"]["completed"]
    );
}
