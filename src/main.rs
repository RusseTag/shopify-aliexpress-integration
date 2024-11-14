mod handlers;
use handlers::product_handler::send_order;
use crate::handlers::product_handler::__path_send_order;

use axum::{
    response::IntoResponse, routing::{get, post}, Router
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Returns the start message
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello message", body = String)
    )
)]
async fn manual_hello() -> impl IntoResponse {
    "Hey there!"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        manual_hello,
        send_order
    ),
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(manual_hello))
        .route("/send_order/:address/:mail", post(send_order))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}