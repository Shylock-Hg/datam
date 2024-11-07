use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
pub struct File {
    id: String,
    sha256: Vec<u8>,
    ipfs_hash: String,
}

impl File {
    pub fn new(id: String, sha256: Vec<u8>, ipfs_hash: String) -> Self {
        Self {
            id,
            sha256,
            ipfs_hash,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_sha256(&self) -> &Vec<u8> {
        &self.sha256
    }

    pub fn get_ipfs_hash(&self) -> &str {
        &self.ipfs_hash
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    // id => file
    files: HashMap<String, File>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn add(&mut self, file: File) {
        self.files.insert(file.id.clone(), file);
    }

    pub fn get(&self, id: &str) -> Option<&File> {
        self.files.get(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<File> {
        self.files.remove(id)
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
