use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use crate::compress::compress;

pub async fn index() -> String {
    // Static HTML form can be kept for browser-based uploads (optional)
    "<html><body><form method='POST' enctype='multipart/form-data'>
        <input type='file' name='fileupload' required>
        <button type='submit'>Upload File</button>
    </form></body></html>"
        .to_string()
}

#[derive(Clone)]
pub struct FileData {
    pub file_data: Vec<u8>,
    pub file_name: String,
}

// Upload multiple files asynchronously and then compress them
pub async fn upload(file_content: Vec<FileData>) {
    let mut file_paths = Vec::new(); // To store file paths for compression

    let mut tasks = Vec::new(); // To store async tasks

    for file in file_content {
        let file_path = Path::new("files").join(&file.file_name);
        let file_path_str = file_path.to_string_lossy().to_string();

        file_paths.push(file_path_str.clone());

        // Spawn asynchronous task for writing file
        let task = tokio::spawn(async move {
            let mut file_handle = TokioFile::create(&file_path)
                .await
                .expect("Failed to open file handle!");

            file_handle
                .write_all(&file.file_data)
                .await
                .expect("Failed to write data to file!");

            println!("✅ File saved: {:?}", file_path);
        });

        tasks.push(task);
    }

    // Wait for all file writes to complete
    for task in tasks {
        task.await.expect("File writing task failed");
    }

    // Compress all saved files asynchronously
    tokio::spawn(async move {
        compress(file_paths);
    });
}
