use axum::{
    routing::get,
    Router,
    response::Html
};

async fn index() -> Html<String>{
    Html("Hello World!".into())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
