use crate::handler::{Context, Handler};
use crate::handler_ipfs::IpfsHandler;
use crate::handler_local::LocalHandler;

pub struct ComposedHandler {
    local: LocalHandler,
    remote: Vec<Box<dyn Handler>>,
}

impl ComposedHandler {
    pub fn new() -> Self {
        Self {
            local: LocalHandler::new(),
            remote: vec![Box::new(IpfsHandler::new())],
        }
    }
}

impl Handler for ComposedHandler {
    fn add(&mut self, ctx: Context) -> Context {
        let mut ctx = ctx;
        for handler in self.remote.iter_mut() {
            ctx = handler.add(ctx);
        }
        self.local.add(ctx)
    }

    fn get(&self, ctx: Context) -> Option<Context> {
        let ctx = self.local.get(ctx);
        if let Some(mut ctx) = ctx {
            for handler in self.remote.iter() {
                ctx = handler.get(ctx.clone()).unwrap_or(ctx);
            }
            Some(ctx)
        } else {
            None
        }
    }

    fn remove(&mut self, ctx: Context) -> Option<Context> {
        let ctx = self.local.remove(ctx);
        if let Some(mut ctx) = ctx {
            for handler in self.remote.iter_mut() {
                ctx = handler.remove(ctx.clone()).unwrap_or(ctx);
            }
            Some(ctx)
        } else {
            None
        }
    }

    fn list(&self) -> String {
        self.local.list()
    }
}
