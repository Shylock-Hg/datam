use std::fs;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::handler_local::LOCAL_DIR;

const ACK_FILE: &str = "ack.json";

// service name => ack
#[derive(Serialize, Deserialize, Debug)]
pub struct Ack{
    github: String,
    google_drive: GoogleSecret,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleSecret {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl GoogleSecret {
    fn new() -> Self {
        Self {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            access_token: "".to_string(),
            refresh_token: "".to_string(),
        }
    }
}

impl Ack {
    fn new() -> Self {
        Self {
            github: "".to_string(),
            google_drive: GoogleSecret::new(),
        }
    }

    pub fn new_by_file() -> Self {
        // mkdir if LOCAL_DIR not exists
        let mut p = simple_home_dir::home_dir().unwrap();
        p.push(LOCAL_DIR);
        if !fs::exists(&p).unwrap() {
            fs::create_dir(&p).unwrap();
        }

        p.push(ACK_FILE);
        let mut f = if !fs::exists(&p).unwrap() {
            fs::File::create_new(p).unwrap()
        } else {
            fs::File::options().write(true).read(true).open(p).unwrap()
        };
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        if content.is_empty() {
            Self::new()
        } else {
            serde_json::from_str(&content).unwrap()
        }
    }

    pub fn github(&self) -> &str {
        &self.github
    }

    pub fn google_drive(&self) -> &GoogleSecret {
        &self.google_drive
    }

}

