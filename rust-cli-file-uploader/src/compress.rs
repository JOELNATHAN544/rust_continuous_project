use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{self, copy, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn compress(files: Vec<String>) {
    // Ensure the "files" directory exists
    let output_dir = Path::new("files");
    fs::create_dir_all(output_dir).expect("Failed to create 'files/' directory");

    println!("\nSelect a compression level between (1, 2, 3):");
    println!("1. Default\n2. Fastest\n3. Best");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
 
    let compression = match choice.trim().parse::<u32>() {
        Ok(1) => Compression::default(),
        Ok(2) => Compression::fast(),
        Ok(3) => Compression::best(),
        _ => {
            eprintln!("Invalid choice. Using default compression.");
            Compression::default()
        }
    };

    for file in files {
        let input_path = PathBuf::from(&file);
        let input_file = File::open(&input_path).expect("Failed to open input file");
        let mut input = BufReader::new(input_file);

        // Generate compressed file path
        let file_name = input_path
            .file_name()
            .expect("Invalid file path")
            .to_string_lossy();
        let output_file_path = output_dir.join(format!("{}.gz", file_name));

        let output_file = File::create(&output_file_path).expect("Failed to create compressed file");
        let mut encoder = GzEncoder::new(output_file, compression);

        let start = Instant::now();
        copy(&mut input, &mut encoder).expect("Failed to copy data");
        let output = encoder.finish().expect("Failed to finish compression");

        println!(
            "\n✅ File: {} compressed successfully!",
            file_name
        );
        println!(
            "📦 Source size: {:?} bytes",
            input.get_ref().metadata().unwrap().len()
        );
        println!(
            "🗜️ Compressed size: {:?} bytes",
            output.metadata().unwrap().len()
        );
        println!("⏳ Compression Time: {:?}", start.elapsed());
    }
}