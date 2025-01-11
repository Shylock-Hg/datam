
use std::path::Path;

use async_trait::async_trait;
use opendal;

use crate::handler::{Context, Handler};
use crate::ack::Ack;


pub struct GdriveHandler {
    op: opendal::Operator,
}

impl GdriveHandler {
    pub fn new(ack: &Ack) -> Self {
        // Pick a builder and configure it.
        let builder = opendal:: services::Gdrive::default().root("/").client_id(&ack.google_drive().client_id).client_secret(&ack.google_drive().client_secret).access_token(&ack.google_drive().access_token);

        // Init an operator
        let op = opendal:: Operator::new(builder).unwrap()
            // Init with logging layer enabled.
            .layer(opendal::layers:: LoggingLayer::default())
            .finish();

        Self {
            op
        }
    }
}


#[async_trait]
impl Handler for GdriveHandler {
    fn name(&self) -> String {
        "Google Drive".to_string()
    }

    async fn add(&mut self, ctx: Context) -> Context {
        let file = Path::new(ctx.get_id());
        let dir = file.parent().unwrap().to_str().unwrap();
        self.op.create_dir(dir).await.unwrap();
        self.op.write(file.to_str().unwrap(), ctx.get_content().clone()).await.unwrap();
        ctx
    }

    async fn get(&self, ctx: Context) -> Option<Context> {
        // Use the operator.
        println!("DEBUG POINT: {:?}", self.op.stat("/assets/images").await.unwrap());


        let buf =  self.op.read( ctx.get_id()).await.unwrap();
        Some(Context::new(
            ctx.get_id().to_string(),
            buf.to_vec(),
            ctx.get_sha256().to_owned(),
            ctx.get_ipfs_digest().to_string()
        ))
    }

    async fn remove(&mut self, ctx: Context) -> Option<Context> {
        self.op.delete(ctx.get_id()).await.unwrap();
        Some(ctx)
    }

    async fn list(&self) -> String {
        "".to_string()
    }
}

