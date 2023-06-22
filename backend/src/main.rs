use axum::{routing::get, Json, Router, http::header::CONTENT_TYPE, extract::Path};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://127.0.0.1:3000".parse().unwrap(),
    ];

    let app = Router::new()
        .merge(routes_hello())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([CONTENT_TYPE])
                .allow_methods([axum::http::Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Server started, listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler))
		.route("/hello/:name", get(handler2))
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

async fn handler2(Path(name): Path<String>) -> Json<Message> {
    Json(Message {
        message: String::from(format!("Hello, {name}!"))
    })
}
