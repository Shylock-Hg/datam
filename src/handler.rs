use async_trait::async_trait;

#[async_trait]
pub trait Handler {
    async fn add(&mut self, ctx: Context) -> Context;
    async fn get(&self, ctx: Context) -> Option<Context>;
    async fn remove(&mut self, ctx: Context) -> Option<Context>;
    async fn list(&self) -> String;
}

#[derive(Clone)]
pub struct Context {
    id: String,
    content: Vec<u8>,
    ipfs_digest: String,
}

impl Context {
    pub fn new(id: String, content: Vec<u8>, ipfs_digest: String) -> Self {
        Self {
            id,
            content,
            ipfs_digest,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_content(&self) -> &Vec<u8> {
        &self.content
    }

    pub fn get_ipfs_digest(&self) -> &str {
        &self.ipfs_digest
    }
}
