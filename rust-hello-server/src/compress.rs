use flate2::write::GzEncoder;
use flate2::Compression;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use std::time::Instant;
use std::io::{self, Write};

pub async fn compress(files: Vec<String>) {
    if files.is_empty() {
        eprintln!("⚠️ No files to compress!");
        return;
    }

    // Ask for compression option
    println!("\nEnter your compression option number (1, 2, 3):");
    println!("1. Default\n2. Fastest\n3. Best");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let compression = match choice.trim().parse::<u32>() {
        Ok(1) => Compression::default(),
        Ok(2) => Compression::fast(),
        Ok(3) => Compression::best(),
        _ => {
            eprintln!("❌ Invalid choice. Using default compression.");
            Compression::default()
        }
    };

    let mut tasks = Vec::new();

    for file in files {
        let compression_level = compression.clone();
        let task = tokio::spawn(async move {
            let file_path = Path::new(&file);
            let output_file_path = format!("{}.gz", file);

            let start = Instant::now();

            // Open input file asynchronously
            let mut input_file = match File::open(&file_path).await {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("❌ Failed to open input file: {}", file);
                    return;
                }
            };

            let mut data = Vec::new();
            if let Err(e) = input_file.read_to_end(&mut data).await {
                eprintln!("❌ Failed to read file {}: {}", file, e);
                return;
            }

            // Create output file asynchronously
            let mut output_file = match File::create(&output_file_path).await {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("❌ Failed to create compressed file: {}", output_file_path);
                    return;
                }
            };

            let mut encoder = GzEncoder::new(Vec::new(), compression_level);
            if let Err(e) = encoder.write_all(&data) {
                eprintln!("❌ Compression failed for {}: {}", file, e);
                return;
            }

            let compressed_data = match encoder.finish() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("❌ Failed to finalize compression for {}: {}", file, e);
                    return;
                }
            };

            if let Err(e) = output_file.write_all(&compressed_data).await {
                eprintln!("❌ Failed to write compressed data for {}: {}", file, e);
            } else {
                println!("✅ File compressed: {} in {:?}", output_file_path, start.elapsed());
            }
        });

        tasks.push(task);
    }

    // Wait for all compression tasks to finish
    for task in tasks {
        let _ = task.await;
    }

    println!("✅ Compression completed for all files.");
}
