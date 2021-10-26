use std::path::PathBuf;

use structopt::StructOpt;

/// An useful tool for generating directory tree
#[derive(StructOpt, Debug)]
#[structopt(name = "Rp Tree")]
pub struct Config {
    /// Only output dir
    #[structopt(short, long)]
    dir_only: bool,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Root dir for generate directory tree
    #[structopt(name = "ROOT_DIR", parse(from_os_str))]
    root_dir: PathBuf,
}

impl Config {
    pub fn is_root_dir(&self) -> bool {
        self.root_dir.is_dir()
    }

    pub fn get_root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    pub fn get_output_file(&self) -> &Option<PathBuf> {
        &self.output
    }

    pub fn dir_only(&self) -> bool {
        self.dir_only
    }
}
