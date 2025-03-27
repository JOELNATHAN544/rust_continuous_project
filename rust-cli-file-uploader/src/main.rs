use crate::loadfile::{index, upload, FileData};
use axum::{routing::get, Router};
use clap::Parser;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, fs::File, io::Read, path::Path};
use tokio::{fs, net::TcpListener};

#[derive(Parser)]
#[command(name = "file_uploader")]
#[command(about = "A simple file uploader", long_about = None)]
struct Cli {
    file_paths: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut file_list = Vec::new();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let dir_path = Path::new("files");
    if !dir_path.exists() {
        fs::create_dir(dir_path)
            .await
            .expect("Failed to create 'files/' directory");
    }

    for file_path in cli.file_paths {
        let mut file = File::open(&file_path).expect("Failed to open file");
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)
            .expect("Failed to read file data");

        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("uploaded_file")
            .to_string();

        file_list.push(FileData {
            file_data,
            file_name,
        });
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    upload(&pool, file_list).await?;

    let app = Router::new().route("/", get(index));
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to serve 'app'!");

    Ok(())
}

mod compress;
mod database;
mod loadfile;