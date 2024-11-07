use crate::handler::{Context, Handler};
use crate::model::{Database, File};
use serde_json;
use sha2::{Digest, Sha256};
use simple_home_dir;
use std::fs;
use std::io::{Read, Seek, Write};

const LOCAL_DIR: &str = ".datam";
const STORE_FILE: &str = "store.json";

pub struct LocalHandler {
    db: Database,
    f: fs::File,
}

impl LocalHandler {
    pub fn new() -> Self {
        // mkdir if LOCAL_DIR not exists
        let mut p = simple_home_dir::home_dir().unwrap();
        p.push(LOCAL_DIR);
        if !fs::exists(&p).unwrap() {
            fs::create_dir(&p).unwrap();
        }

        p.push(STORE_FILE);
        let mut f = if !fs::exists(&p).unwrap() {
            fs::File::create_new(p).unwrap()
        } else {
            fs::File::options().write(true).read(true).open(p).unwrap()
        };
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        let db = if content.is_empty() {
            Database::new()
        } else {
            serde_json::from_str(&content).unwrap()
        };

        Self { db, f }
    }
}

impl Handler for LocalHandler {
    fn add(&mut self, ctx: Context) -> Context {
        let mut hasher = Sha256::new();
        hasher.update(ctx.get_content());
        let sha256 = hasher.finalize();

        let file = File::new(
            ctx.get_id().to_owned(),
            sha256.to_vec(),
            ctx.get_ipfs_digest().to_owned(),
        );
        self.db.add(file);
        ctx
    }

    fn get(&self, ctx: Context) -> Option<Context> {
        let res = self.db.get(ctx.get_id());
        res.map(|f| {
            Context::new(
                f.get_id().to_owned(),
                f.get_sha256().to_owned(),
                f.get_ipfs_hash().to_owned(),
            )
        })
    }

    fn remove(&mut self, ctx: Context) -> Option<Context> {
        let res = self.db.remove(ctx.get_id());
        res.map(|f| {
            Context::new(
                f.get_id().to_owned(),
                f.get_sha256().to_owned(),
                f.get_ipfs_hash().to_owned(),
            )
        })
    }

    fn list(&self) -> String {
        self.db.to_string()
    }
}

impl Drop for LocalHandler {
    fn drop(&mut self) {
        let content = serde_json::to_string_pretty(&self.db).unwrap();
        self.f.rewind().unwrap();
        self.f.set_len(0).unwrap();
        self.f.write_all(content.as_bytes()).unwrap();
    }
}
