use crate::compress::compress;
use axum::{extract::Multipart, response::Html};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

pub async fn upload(mut multipart: Multipart) {
    let file_paths = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = Vec::new();

    while let Some(field) = multipart.next_field().await.expect("Failed to get next field!") {
        if let Some(file_name) = field.file_name().map(String::from) {
            let file_path = format!("files/{}", file_name);
            let data = field.bytes().await.expect("Failed to read file bytes");

            let file_paths_clone = Arc::clone(&file_paths);

            let task = tokio::spawn(async move {
                let mut file_handle = match TokioFile::create(&file_path).await {
                    Ok(f) => f,
                    Err(_) => {
                        eprintln!("❌ Failed to open file handle for {}", file_path);
                        return;
                    }
                };

                if let Err(e) = file_handle.write_all(&data).await {
                    eprintln!("❌ Failed to write data to {}: {}", file_path, e);
                    return;
                }

                println!("✅ File uploaded: {}", file_path);

                let mut paths = file_paths_clone.lock().await;
                paths.push(file_path);
            });

            tasks.push(task);
        }
    }

    // Wait for all uploads to complete
    for task in tasks {
        let _ = task.await;
    }

    // Start compression only after all files are uploaded
    let paths = file_paths.lock().await.clone();
    compress(paths).await;
}
