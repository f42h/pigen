mod core;
use core::pigen::Pigen;
use std::{env, process::exit};

fn main() {
    let mut args = env::args();
    let password_len = match args.nth(1) {
        Some(len) => len,
        None => {
            println!("Please provide a password length!");
            exit(-1);
        }
    };

    if let Ok(len) = password_len.parse::<usize>() {
        if len == 0 || len > 128 {
            eprintln!("Please use a valid size (up to 128 characters)");
            exit(-1);
        }

        let session = Pigen::new(len);
        session.generate(|password| {
            println!();
            println!("Generated password (Size: {} chars): {}", len, password);
            println!();
        });
    }
}
