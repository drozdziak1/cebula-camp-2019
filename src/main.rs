use rand::Rng;
use rpassword::prompt_password_stdout;
use sha3::{Digest, Keccak256};

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
        "Hello {}, you entered a pass {} chars long which hashes to {:?}",
        username,
        password.len(),
        bytes2string(Keccak256::digest(password.as_bytes()).as_slice())
    );
}

/// Hexdumps the supplied slice and returns the stringified result; good for hashes
pub fn bytes2string(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        out.push_str(&format!("{:x}", byte));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes2string_test() {
        let cases = vec![(vec![], ""), (vec![97], "61"), (vec![0, 0], "0000")];

        for (input, expected) in cases {
            assert_eq!(bytes2string(input.as_slice()).as_str(), expected);
        }
    }
}
