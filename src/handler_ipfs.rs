use std::io::Cursor;

use crate::handler::{Context, Handler};
use async_trait::async_trait;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use log;

pub struct IpfsHandler {
    client: IpfsClient,
}

impl IpfsHandler {
    pub fn new() -> Self {
        Self {
            client: IpfsClient::default(),
        }
    }
}

#[async_trait]
impl Handler for IpfsHandler {
    async fn add(&mut self, ctx: Context) -> Context {
        let cursor = Cursor::new(ctx.get_content().to_owned());
        let res = self.client.add(cursor).await.unwrap();
        log::info!("Ipfs digest: {}", res.hash);
        Context::new(
            ctx.get_id().to_owned(),
            ctx.get_content().to_owned(),
            res.hash,
        )
    }

    async fn get(&self, ctx: Context) -> Option<Context> {
        let content = self
            .client
            .cat(ctx.get_ipfs_digest())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await;
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

    async fn remove(&mut self, ctx: Context) -> Option<Context> {
        Some(ctx)
    }

    async fn list(&self) -> String {
        "".to_string()
    }
}
