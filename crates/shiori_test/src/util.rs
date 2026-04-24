use axum::body::{Body, Bytes};
use futures_util::FutureExt;
use http::{Method, Request, header};
use tower::ServiceExt;

use crate::{mock_request::MockRequest, response::Response, test_app::TestApp};

#[allow(async_fn_in_trait)]
pub trait RequestHelper {
    fn request_builder(&self, method: Method, path: &str) -> MockRequest;
    fn app(&self) -> &TestApp;

    fn run<T>(&self, request: Request<impl Into<Body>>) -> impl Future<Output = Response<T>> {
        let app = self.app();
        let request = request.map(Into::into);

        async fn inner(app: &TestApp, request: Request<Body>) -> axum::response::Response<Bytes> {
            let router = app.router().clone();

            let axum_response = router.oneshot(request).await.unwrap();

            let (parts, body) = axum_response.into_parts();
            let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
            axum::response::Response::from_parts(parts, bytes)
        }

        inner(app, request).map(Response::new)
    }

    /// Create a get request
    fn get_request(&self, path: &str) -> MockRequest {
        self.request_builder(Method::GET, path)
    }

    /// Create a POST request
    fn post_request(&self, path: &str) -> MockRequest {
        self.request_builder(Method::POST, path)
    }

    /// Issue a GET request
    async fn get<T>(&self, path: &str) -> Response<T> {
        self.run(self.get_request(path)).await
    }

    /// Issue a GET request that includes query parameters
    async fn get_with_query<T>(&self, path: &str, query: &str) -> Response<T> {
        let path_and_query = format!("{path}?{query}");
        let request = self.request_builder(Method::GET, &path_and_query);
        self.run(request).await
    }

    /// Issue a POST request
    async fn post<T>(&self, path: &str, body: impl Into<Bytes>) -> Response<T> {
        let request = self
            .request_builder(Method::POST, path)
            .with_body(body.into());

        self.run(request).await
    }

    /// Issue a PUT request
    async fn put<T>(&self, path: &str, body: impl Into<Bytes>) -> Response<T> {
        let request = self
            .request_builder(Method::PUT, path)
            .with_body(body.into());

        self.run(request).await
    }

    /// Issue a PATCH request
    async fn patch<T>(&self, path: &str, body: impl Into<Bytes>) -> Response<T> {
        let request = self
            .request_builder(Method::PATCH, path)
            .with_body(body.into());

        self.run(request).await
    }

    /// Issue a DELETE request
    async fn delete<T>(&self, path: &str) -> Response<T> {
        let request = self.request_builder(Method::DELETE, path);
        self.run(request).await
    }
}

pub fn req(method: Method, path: &str) -> MockRequest {
    Request::builder()
        .method(method)
        .uri(path)
        .header(header::USER_AGENT, "test")
        .body(Bytes::new())
        .unwrap()
}
