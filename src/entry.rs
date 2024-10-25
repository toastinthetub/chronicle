use serde::{Deserialize, Serialize};
use std::{fs, path::Path}

use crate::encrypt::{encrypt_entry, decrypt_entry};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub title: String,
    pub timestamp: String, // let timestamp: String = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    pub encrypted_contents: String,
}

impl Entry {
    pub fn construct_from_str(s: &str, t: &str) -> Result<Entry, Box<dyn std::error::Error>> {
        let title: String = t.to_owned();
        let timestamp: String = chrono::Utc::now().format("[%Y-%m-%d %H:%M]").to_string();
        let encrypted_contents = s.to_owned();
        Ok(Self {
            title,
            timestamp,
            encrypted_contents,
        })
    }
    pub fn encrypt_self(&mut self, password: String) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: implement encrypted byte sig condition 
        Ok(())
    }
    // TODO: figure out what else to do with this struct
}

pub struct DiaryEntries {
    pub entries: Vec<Entry>
}

impl DiaryEntries {
    fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }
    pub fn safe_open() -> Result<Self, Box<dyn std::error::Error>> {
        todo!() // TODO: implement safe_open()
    }
    pub fn safe_close() -> Result<(), Box<dyn std::error::Error>> {
        todo!() // TODO: implement safe_close()
        
    }
    // TODO: implement serialization of entires
    // TODO: implement deserialization of entries
    // TODO: implement appending entries to existing diary
}
