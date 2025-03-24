use crate::loadfile::{index, upload, FileData};
use axum::{routing::get, Router};
use clap::Parser;
use std::{fs::File, io::Read, path::Path};
//use tokio::fs;
use tokio::{fs, net::TcpListener};

#[derive(Parser)]
#[command(name = "file_uploader")]
#[command(about = "A simple file uploader", long_about = None)]
struct Cli {
    /// Paths to the files to upload (supports multiple)
    file_paths: Vec<String>,
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    let mut file_list = Vec::new();

    // Ensure the 'files/' directory exists
    let dir_path = Path::new("files");
    if !dir_path.exists() {
        fs::create_dir(dir_path)
            .await
            .expect("Failed to create 'files/' directory");
    }

    // Read each file and store data
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

        // Store in vector
        file_list.push(FileData {
            file_data,
            file_name,
        });
    }

    // Upload all files
    upload(file_list).await;

    // Set up the Axum router
    let app = Router::new().route("/", get(index));

    // Bind the server
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");

    println!("🚀 Server running on http://localhost:3000");

    // Serve the application
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to serve 'app'!");
}

mod compress;
mod loadfile;
