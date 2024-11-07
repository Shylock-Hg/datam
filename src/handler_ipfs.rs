use std::io::Cursor;

use crate::handler::{Context, Handler};
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use log;
use tokio::runtime::Runtime;

pub struct IpfsHandler {
    client: IpfsClient,
    runtime: Runtime,
}

impl IpfsHandler {
    pub fn new() -> Self {
        Self {
            client: IpfsClient::default(),
            runtime: Runtime::new().unwrap(),
        }
    }
}

impl Handler for IpfsHandler {
    fn add(&mut self, ctx: Context) -> Context {
        let cursor = Cursor::new(ctx.get_content().to_owned());
        let res = self.runtime.block_on(self.client.add(cursor)).unwrap();
        log::info!("Ipfs digest: {}", res.hash);
        Context::new(
            ctx.get_id().to_owned(),
            ctx.get_content().to_owned(),
            res.hash,
        )
    }

    fn get(&self, ctx: Context) -> Option<Context> {
        let content = self
            .client
            .cat(ctx.get_ipfs_digest())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat();
        let content = self.runtime.block_on(content);
        content
            .map(|content| {
                Context::new(
                    ctx.get_id().to_owned(),
                    content,
                    ctx.get_ipfs_digest().to_owned(),
                )
            })
            .ok()
    }

    fn remove(&mut self, ctx: Context) -> Option<Context> {
        Some(ctx)
    }

    fn list(&self) -> String {
        "".to_string()
    }
}
