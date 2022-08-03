use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .fallback(fallback.into_service())
        .route("/", get(hello))
        .route("/demo-status", get(demo_status))
        .route("/hello.html", get(hello_html))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn hello() -> String {
    "hello cruel world!".into()
}

async fn hello_html() -> Html<&'static str> {
    include_str!("hello.html").into()
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route for {}", uri),
    )
}

async fn demo_status() -> (StatusCode, String) {
    (StatusCode::OK, "Everything is okay".to_string())
}

async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is {:?}", uri)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!(" signal shutdown");
}

async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::decode(png).unwrap(),
    )
}
