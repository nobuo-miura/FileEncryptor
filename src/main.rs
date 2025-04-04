use std::env;
use std::fs;
mod crypto;
mod gui;

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {} encrypt <input> <output> <password>", program);
    eprintln!("  {} decrypt <input> <output> <password>", program);
    eprintln!("  {} gui", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "encrypt" => {
            if args.len() != 5 {
                print_usage(&args[0]);
                return;
            }
            let input = &args[2];
            let output = &args[3];
            let password = &args[4];

            let plaintext = fs::read(input).expect("Failed to read input file");
            let encrypted = crypto::encrypt_data(&plaintext, password);
            fs::write(output, encrypted).expect("Failed to write output file");
            println!("Encryption successful!");
        }
        "decrypt" => {
            if args.len() != 5 {
                print_usage(&args[0]);
                return;
            }
            let input = &args[2];
            let output = &args[3];
            let password = &args[4];

            let encrypted = fs::read(input).expect("Failed to read input file");
            match crypto::decrypt_data(&encrypted, password) {
                Ok(decrypted) => {
                    fs::write(output, decrypted).expect("Failed to write output file");
                    println!("Decryption successful!");
                }
                Err(e) => eprintln!("Decryption failed: {}", e),
            }
        }
        "gui" => {
            gui::launch_gui();
        }
        _ => {
            print_usage(&args[0]);
        }
    }
}