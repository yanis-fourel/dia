use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Dia file to run
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("File is {:?}", cli.file);
}
