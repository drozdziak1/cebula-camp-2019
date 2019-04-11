use chrono::prelude::*;
use rpassword::prompt_password_stdout;
use sha3::{Digest, Keccak256};

use std::io::{self, Write};

mod lib;
mod type_system;

use lib::*;
use type_system::{Doge, Meme, MemeState, Pepe, Pope};

#[derive(Debug)]
pub enum MemeError {
    ZombiePope,
}

fn zobacz_memy() -> Result<(), MemeError> {
    let jp2 = Pope::new(
        "Jan Paweł II",
        MemeState::Dead(Utc.ymd(2005, 4, 2).and_hms(21, 37, 0)),
    );

    match jp2.state() {
        MemeState::Alive => return Err(MemeError::ZombiePope),
        MemeState::Dead(when) => println!("{} died at {}", jp2.name(), when.naive_local()),
    }

    let mut francis = Pope::new("Franciszek", MemeState::Alive);

    dbg!(&francis);

    francis.kill();

    println!("After kill(): {:?}", francis);

    let doge = Doge {};

    dbg!(doge);

    let reeee = Pepe {
        name: "Reeeeeeeeeee Pepe".to_owned(),
        state: MemeState::Alive,
    };

    // kill() won't work, because it's neither implemented in the trait nor the Pepe struct
    //reeee.kill();

    dbg!(reeee);

    Ok(())
}

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

    let hash = bytes2string(Keccak256::digest(password.as_bytes()).as_slice());
    println!(
        "Hello {}, you entered a pass {} chars long which hashes to {}",
        username,
        password.len(),
        hash
    );
    drop(password);

    println!("Username and hash together: {}", my_concat(username, &hash));

    zobacz_memy().unwrap();
}
