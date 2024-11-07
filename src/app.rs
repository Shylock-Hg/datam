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
        #[arg(long, help = "Path to file to add.")]
        path: String,
    },
    // get a file
    Get {
        #[arg(long, help = "ID of file to get.")]
        id: String,
    },
    // list all files
    List {
        #[arg(long, help = "Show details of files.")]
        verbose: bool,
    },
    Remove {
        #[arg(long, help = "ID of file to remove.")]
        id: String,
    },
}
