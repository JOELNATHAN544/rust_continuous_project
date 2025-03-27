// use crate::compress::compress;
// use crate::database::{self, Process};
// use dotenvy::dotenv;
// use sqlx::postgres::PgPoolOptions;
// use std::path::Path;
// use tokio::fs::File as TokioFile;
// use tokio::io::AsyncWriteExt;
// use std::env;

// pub async fn index() -> String {
//     "<html><body><form method='POST' enctype='multipart/form-data'>
//         <input type='file' name='fileupload' required>
//         <button type='submit'>Upload File</button>
//     </form></body></html>".to_string()
// }

// #[derive(Clone)]
// pub struct FileData {
//     pub file_data: Vec<u8>,
//     pub file_name: String,
// }

// pub async fn upload(file_content: Vec<FileData>) {
//     let mut file_paths = Vec::new();
//     let mut tasks = Vec::new();
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     for file in file_content {
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&database_url)
//             .await.unwrap();
//         let file_path = Path::new("files").join(&file.file_name);
//         let file_path_str = file_path.to_string_lossy().to_string();
        
//         let process = database::database(&pool, &file_path_str, "File uploaded successfully").await;
//         println!("File {:?} sent in database", process);
//         file_paths.push(file_path_str.clone());

//         let task = tokio::spawn(async move {
//             let mut file_handle = TokioFile::create(&file_path)
//                 .await
//                 .expect("Failed to open file handle!");
//             file_handle.write_all(&file.file_data)
//                 .await
//                 .expect("Failed to write data to file!");
//             println!("✅ File saved: {:?}", file_path);
//         });
//         tasks.push(task);
//     }

//     for task in tasks {
//         task.await.expect("File writing task failed");
//     }

//     tokio::spawn(async move {
//         compress(file_paths);
//     });
// }

use crate::compress::compress;
use crate::database::{self, Process};
use std::path::Path;
// use sqlx::postgres::PgPoolOptions;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use sqlx::PgPool;
use tracing::info;

pub async fn index() -> String {
    r#"
    <html>
        <body>
            <h1>File Uploader</h1>
            <form method='POST' enctype='multipart/form-data'>
                <input type='file' name='fileupload' multiple required>
                <button type='submit'>Upload File(s)</button>
            </form>
        </body>
    </html>
    "#.to_string()
}

#[derive(Clone)]
pub struct FileData {
    pub file_data: Vec<u8>,
    pub file_name: String,
}

pub async fn upload(pool: &PgPool, file_content: Vec<FileData>) -> anyhow::Result<()> {
    let mut file_paths = Vec::new();
    let mut tasks = Vec::new();
    // dotenvy::dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL")?;
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&database_url)
    //     .await?;

    for file in file_content {
        let file_path = Path::new("files").join(&file.file_name);
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Save to database first
        let process = database::database(pool, &file_path_str, "uploaded file successfully").await?;
        info!("File recorded in database: {:?}", process);
        file_paths.push(file_path_str.clone());

        // Then save to filesystem
        let task = tokio::spawn(async move {
            if let Some(parent) = file_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let mut file_handle = TokioFile::create(&file_path).await?;
            file_handle.write_all(&file.file_data).await?;
            file_handle.sync_all().await?;
            info!("✅ File saved: {}", file_path.display());
            Ok::<_, anyhow::Error>(())
        });
        tasks.push(task);
    }

    // Wait for all file operations to complete
    for task in tasks {
        task.await??;
    }

    // Compress files in background
    tokio::spawn(async move {
        compress(file_paths);
    });

    Ok(())
}