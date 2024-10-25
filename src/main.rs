mod arg;
mod encrypt;
mod entry;
mod state;
mod terminal;

use chrono::{DateTime, Utc};
use encrypt::{decrypt_entry, encrypt_entry, EncryptionError};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: implement argument parsing [arg.rs]
    println!("hello world.");
    Ok(())
}
