use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use chrono::Local;

use crate::encrypt::{decrypt_entry, encrypt_entry, EncryptionError};

pub struct Entry {
    pub title: String,
    pub contents: String,
    pub edit_buffer: String,
}

impl Entry {
    pub fn no_entry() -> Self {
        Self {
            title: String::new(),
            contents: String::new(),
            edit_buffer: String::new(),
        }
    }

    pub fn create_new_entry() -> Result<Self, Box<dyn std::error::Error>> {
        let title: String = format!("UNTITLED_{}", Local::now().format("%m-%d-%Y"));
        let contents: String = String::new();
        let edit_buffer: String = String::new();
        Ok(Self {
            title,
            contents,
            edit_buffer,
        })
    }

    pub fn construct_from_str(s: &str, t: &str) -> Result<Entry, Box<dyn std::error::Error>> {
        let timestamp: String = chrono::Utc::now().format("[%Y-%m-%d %H:%M]").to_string();
        let title: String = format!("{}{}", t.to_owned(), timestamp);
        let contents = s.to_owned();
        let edit_buffer = contents.clone(); // create a seperate clone of the contents, to be edited.
        Ok(Self {
            title,
            contents,
            edit_buffer, // edit buffer should always be empty when serialized
        })
    }

    pub fn write_edit_buffer(&mut self) {
        if self.edit_buffer == self.contents {
            return;
        }

        self.contents.replace_range(.., &self.edit_buffer); // write the edit buffer to the contents
    }

    pub fn encrypt_self(&mut self, password: String) -> Result<(), EncryptionError> {
        self.write_edit_buffer(); // ensure edit buffer is written to contents
        self.contents = match encrypt_entry(&password, &self.contents) {
            Ok(encrypted_contents) => encrypted_contents,
            Err(e) => {
                eprintln!(
                    "failed to encrypt entry {} because of error: {}",
                    &self.title, &e
                );
                return Err(e);
            }
        };
        Ok(())
    }
    pub fn decrypt_self(&mut self, password: String) -> Result<(), EncryptionError> {
        match decrypt_entry(&password, &self.contents) {
            Ok(decrypted_contents) => self.contents.replace_range(.., &decrypted_contents),
            Err(e) => {
                eprintln!(
                    "failed to decrypt entry {} because of error: {:?}",
                    &self.title, &e
                );
                return Err(e);
            }
        }
        Ok(())
    }
    // TODO: figure out what else to do with this struct
}

#[derive(Serialize, Deserialize)]
pub struct SerializableEntry {
    pub title: String,
    pub contents: String,
}

impl SerializableEntry {
    pub fn from_entry(entry: Entry) -> Self {
        Self {
            title: entry.title,
            contents: entry.contents,
        }
    }
    pub fn to_entry(self) -> Entry {
        Entry {
            title: self.title,
            contents: self.contents,
            edit_buffer: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DiaryEntries {
    pub entries: Vec<SerializableEntry>,
}

impl DiaryEntries {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn safe_open(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: implement safe_open()

        let json_str: String = match std::fs::read_to_string(filepath) {
            Ok(json_str) => json_str,
            Err(e) => {
                eprintln!("failed to safe open due to this error: {}", e);
                return Ok(DiaryEntries::new());
            }
        };
        let deserialized: DiaryEntries = match serde_json::from_str(&json_str) {
            Ok(deserialized) => deserialized,
            Err(e) => {
                eprintln!("failed to safe open due to this error: {}", e);
                return Ok(DiaryEntries::new());
            }
        };

        Ok(deserialized)
    }
    pub fn safe_close() -> Result<(), Box<dyn std::error::Error>> {
        todo!() // TODO: implement safe_close()
    }
    // TODO: implement serialization of entires
    // TODO: implement deserialization of entries
    // TODO: implement appending entries to existing diary
}
