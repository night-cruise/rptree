use std::path::PathBuf;

use structopt::StructOpt;

/// An useful tool for generating directory tree
#[derive(StructOpt, Debug)]
#[structopt(name = "Rp Tree")]
pub struct Config {
    /// Only output dir
    #[structopt(short, long)]
    dir_only: bool,

    /// Prints to file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Prints color, only supports green, blue and red
    #[structopt(short, long)]
    color: Option<String>,

    /// Root dir for generate directory tree
    #[structopt(name = "ROOT_DIR", parse(from_os_str))]
    root_dir: PathBuf,

    /// Directories to be filtered out
    #[structopt(short = "f", long = "filter", use_delimiter = true)]
    filter: Vec<String>,

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

    pub fn get_color(&self) -> Option<&str> {
        if let Some(color) = &self.color {
            Some(color)
        } else {
            None
        }
    }

    pub fn get_filter(&self) -> &Vec<String> {
        &self.filter
    }

    
}
