use clap;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCmd,
}

#[derive(clap::Subcommand)]
pub enum SubCmd {
    // add a file
    Add {
        #[arg(long)]
        path: String,
    },
    // get a file
    Get {
        #[arg(long)]
        id: String,
    },
    // list all files
    List {
        #[arg(long, help = "Show details of files.")]
        verbose: bool,
    },
    Remove {
        #[arg(long)]
        id: String,
    },
}
