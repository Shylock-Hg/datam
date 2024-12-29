use std::path::PathBuf;

use clap::Parser;
use simple_logger;

mod app;
mod github;
mod handler;
mod handler_composed;
mod handler_ipfs;
mod handler_local;
mod model;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let arg = app::Args::parse();
    match arg.cmd {
        app::SubCmd::Add { path } => {
            let path = PathBuf::from(path).canonicalize().unwrap();
            let home_dir = simple_home_dir::home_dir().unwrap();
            let content = tokio::fs::read(&path).await.unwrap();
            let mut handler = handler_composed::ComposedHandler::new();
            let id = path.strip_prefix(home_dir).unwrap();
            let ctx = handler::Context::new(
                id.to_str().unwrap().to_owned(),
                content,
                vec![],
                "".to_string(),
            );
            handler.add(ctx).await;
        }
        app::SubCmd::Get { id } => {
            let handler = handler_composed::ComposedHandler::new();
            let ctx = handler::Context::new(id.clone(), vec![], vec![], "".to_string());
            let res = handler.get(ctx).await;
            let mut f = simple_home_dir::home_dir().unwrap();
            f.push(&id);
            if let Some(res) = res {
                tokio::fs::File::create(&f).await.unwrap();
                tokio::fs::write(f, res.get_content()).await.unwrap();
            } else {
                eprintln!("Can not find file with id {}", id);
                std::process::exit(1);
            }
        }
        app::SubCmd::List { .. } => {
            let handler = handler_composed::ComposedHandler::new();
            println!("{}", handler.list().await);
        }
        app::SubCmd::Remove { id } => {
            let mut handler = handler_composed::ComposedHandler::new();
            let ctx = handler::Context::new(id, vec![], vec![], "".to_string());
            handler.remove(ctx).await;
        }
        app::SubCmd::SyncGH { token } => {
            github::Github::new_token(token).sync_github().await;
        }
    }
}
