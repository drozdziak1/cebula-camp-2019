use rpassword::prompt_password_stdout;

use std::io::{self, Write};

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
        "Hello {}, you entered a pass {} chars long",
        username,
        password.len()
    );
}
