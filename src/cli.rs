use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CompilerArgs {
    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,

    #[arg(short)]
    pub output: Option<PathBuf>,
}
