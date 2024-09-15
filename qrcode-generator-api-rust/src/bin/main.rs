use axum::extract::Query;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use image::{ImageBuffer, ImageFormat, Luma};
use qrcode::QrCode;
use serde::Deserialize;
use std::io::Cursor;
use axum::body::Body;
use hyper::header;

#[tokio::main]
async fn main() {
    // Compose the routes
    let app = Router::new()
        .route("/api/v1/generate", get(generate));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct Params {
    data: String
}

async fn generate(Query(params): Query<Params>) -> impl IntoResponse {
    let qr_code = QrCode::new(params.data).unwrap();

    let image = qr_code.render::<Luma<u8>>().min_dimensions(1000, 1000).build();

    let mut buffer = Cursor::new(Vec::new());

    ImageBuffer::from_fn(image.width(), image.height(), |x, y| image[(x, y)])
        .write_to(&mut buffer, ImageFormat::Png)
        .unwrap();

    Response::builder()
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::from(buffer.into_inner()))
        .unwrap()
}