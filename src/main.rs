use rand::Rng;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Seek};
use std::path::Path;
use std::process;

fn shred_file(file_path: &str, iterations: i32) -> io::Result<()> {
    let file_size = fs::metadata(file_path)?.len() as u64;
    let mut rng = rand::thread_rng();

    for iteration in 0..iterations {
        let mut file = OpenOptions::new()
            .write(true)
            .open(file_path)?;

        let pattern: Vec<u8> = (0..file_size)
            .map(|_| rng.gen())
            .collect();

        for _ in 0..10 {
            let offset = rng.gen_range(0..=file_size - pattern.len() as u64);
            file.seek(io::SeekFrom::Start(offset))?;

            let bytes_written = file.write(&pattern)?;
            file.flush()?;

            if bytes_written != pattern.len() {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "Failed to write entire pattern",
                ));
            }
        }

        file.set_len(file_size)?;

        // Print progress information for the user
        let percent_complete = (iteration + 1) as f32 / iterations as f32 * 100.0;
        println!("Shredding file... {:.1}% complete", percent_complete);
    }

    Ok(())
}





 fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: shredder -f <file location> -i <number of iterations>\n\
                  The more times you shred the file, the harder it will be to recover.\n\
                  Default number of iterations is 3.");
        process::exit(0);
    }

    if args.len() != 5 {
        println!("Invalid command-line arguments.");
        process::exit(1);
    }

    let file_path = &args[2];
    let iterations: i32 = args[4].parse().unwrap_or_else(|_| {
        println!("Invalid number of iterations.");
        process::exit(1);
    });

    if iterations < 1 || iterations > 25 {
        println!("Invalid number of iterations.");
        process::exit(1);
    }

    if !Path::new(file_path).exists() {
        println!("File not found.");
        process::exit(1);
    }

    let allowed_extensions = [
        "txt", "docx", "pdf", "xlsx", "doc", "pptx", "ppt", "xls", "csv", "jpg", "jpeg", "png", "gif", "bmp", "mp3", "wav", "mp4", "avi", "mov", "exe", "zip", "rar"
    ];

    let file_extension = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !allowed_extensions.contains(&file_extension.as_str()) {
        println!("Invalid file extension.");
        println!("File extension: {}", file_extension);
        process::exit(1);
    }

        // Check if file has a valid size
        let file_size = fs::metadata(file_path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);

    if file_size < 1 || file_size > 100_000_000 {
        println!("Invalid file size.");
        return;
    }

    // Check if file is not read-only
    let is_read_only = fs::metadata(file_path)
        .map(|metadata| metadata.permissions().readonly())
        .unwrap_or(true);

    if is_read_only {
        println!("You do not have permission to shred this file.");
        return;
    }

    // Check if file path is valid
    let invalid_file_name_chars: &[char] = &[
        '<', '>', ':', '"', '/', '\\', '|', '?', '*',
    ];

    if invalid_file_name_chars.iter().any(|c| file_path.contains(*c)) {
        println!("Invalid file path.");
        return;
    }

    if let Err(e) = shred_file(file_path, iterations) {
        println!("Error shredding file: {}", e);
        process::exit(1);
    }
    
    fs::remove_file(file_path).unwrap();

    println!("File : {} -> shredded successfully.", file_path);

}

