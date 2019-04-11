use rpassword::prompt_password_stdout;
use sha3::{Digest, Keccak256};

use std::io::{self, Write};

mod lib;

use lib::bytes2string;

fn main() {
    let mut username = String::new();
    while username.is_empty() {
        print!("Username: ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        stdin.read_line(&mut username).unwrap();
        username = username.trim().to_owned();
    }

    let mut password = String::new();
    while password.is_empty() {
        password = prompt_password_stdout("Password: ").unwrap();
    }

    println!(
        "Hello {}, you entered a pass {} chars long which hashes to {}",
        username,
        password.len(),
        bytes2string(Keccak256::digest(password.as_bytes()).as_slice())
    );
}
