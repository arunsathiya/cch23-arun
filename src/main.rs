use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn minus_one() -> Response<String> {
    Response::builder()
        .status(500)
        .body("-1".to_string())
        .unwrap()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(minus_one));

    Ok(router.into())
}
