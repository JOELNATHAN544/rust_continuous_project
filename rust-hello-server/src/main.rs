use axum::{routing::get, Router};
use loadfile::{index, upload};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");


    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}

mod loadfile;
mod compress;
//main.rs