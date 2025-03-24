extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    println!("\nEnter your compression option number (1, 2, 3) :\n1. Default\n2. Fastest\n3. Best\n");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim().parse::<u32>().unwrap() {
        1 => {
            let mut encoder = GzEncoder::new(output, Compression::default());
            let start = Instant::now();
            copy(&mut input, &mut encoder).unwrap();
            let output = encoder.finish().unwrap();
            println!(
                "Source len: {:?}",
                input.get_ref().metadata().unwrap().len()
            );
            println!("Target len: {:?}", output.metadata().unwrap().len());
            println!("Elapsed: {:?}", start.elapsed());
        }
        2 => {
            let mut encoder = GzEncoder::new(output, Compression::fast());
            let start = Instant::now();
            copy(&mut input, &mut encoder).unwrap();
            let output = encoder.finish().unwrap();
            println!(
                "Source len: {:?}",
                input.get_ref().metadata().unwrap().len()
            );
            println!("Target len: {:?}", output.metadata().unwrap().len());
            println!("Elapsed: {:?}", start.elapsed());
        }

        3 => {
            let mut encoder = GzEncoder::new(output, Compression::best());
            let start = Instant::now();
            copy(&mut input, &mut encoder).unwrap();
            let output = encoder.finish().unwrap();
            println!(
                "Source len: {:?}",
                input.get_ref().metadata().unwrap().len()
            );
            println!("Target len: {:?}", output.metadata().unwrap().len());
            println!("Elapsed: {:?}", start.elapsed());
        }
        _ => {
            println!("Invalid choice");
        }
    }
}
// let mut encoder = GzEncoder::new(output, Compression::default());
// let start = Instant::now();
// copy(&mut input, &mut encoder).unwrap();
// let output = encoder.finish().unwrap();
// println!(
//     "Source len: {:?}",
//     input.get_ref().metadata().unwrap().len()
// );
// println!("Target len: {:?}", output.metadata().unwrap().len());
// println!("Elapsed: {:?}", start.elapsed());
// extern crate flate2;

// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::env::args;
// use std::fs::{create_dir_all, File};
// use std::io::{copy, BufReader};
// use std::path::Path;
// use std::time::Instant;

// fn main() {
//     if args().len() != 3 {
//         eprintln!("Usage: <source> <target>");
//         return;
//     }

//     // Get the source file path
//     let source_path = args().nth(1).unwrap();

//     // Get the output file path
//     let target_file = args().nth(2).unwrap();

//     // Ensure the "files" directory exists before writing
//     let output_directory = Path::new("files");
//     create_dir_all(output_directory).expect("Failed to create directory");

//     // Create the full output path inside "files/" directory
//     let output_path = output_directory.join(target_file);

//     // Open the input file
//     let mut input = BufReader::new(File::open(&source_path).expect("Failed to open source file"));

//     // Create the compressed output file
//     let output = File::create(output_path).expect("Failed to create output file");

//     // Create a gzip encoder
//     let mut encoder = GzEncoder::new(output, Compression::default());

//     // Start the timer
//     let start = Instant::now();

//     // Copy the data from input to encoder (compression process)
//     copy(&mut input, &mut encoder).expect("Failed to compress file");

//     // Finish writing compressed data and get the final output file
//     let output = encoder.finish().expect("Failed to finish compression");

//     // Print source and target file sizes
//     println!(
//         "Source len: {:?} bytes",
//         input.get_ref().metadata().unwrap().len()
//     );
//     println!("Target len: {:?} bytes", output.metadata().unwrap().len());
//     println!("Elapsed: {:?}", start.elapsed());
// }
