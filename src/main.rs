use std::path::PathBuf;

use clap::Parser;
use simple_logger;

mod app;
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
            let path = PathBuf::from(path);
            let filename = path.file_name().unwrap().to_str().unwrap();
            let content = std::fs::read(&path).unwrap();
            let mut handler = handler_composed::ComposedHandler::new();
            let ctx = handler::Context::new(filename.to_owned(), content, vec![], "".to_string());
            handler.add(ctx).await;
        }
        app::SubCmd::Get { id } => {
            let handler = handler_composed::ComposedHandler::new();
            let ctx = handler::Context::new(id.clone(), vec![], vec![], "".to_string());
            let res = handler.get(ctx).await;
            if let Some(res) = res {
                std::fs::File::create(&id).unwrap();
                std::fs::write(id, res.get_content()).unwrap();
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
    }
}
