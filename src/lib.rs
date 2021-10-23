use std::path::{Path, PathBuf};
use structopt::StructOpt;

const PIPE: &str = "|";
const ELBOW: &str = "└──";
const TEE: &str = "├──";
const PIPE_PREFIX: &str = "│   ";
const SPACE_PREFIX: &str = "    ";


/// An useful tool for generating directory tree
#[derive(StructOpt, Debug)]
#[structopt(name = "Rp Tree")]
pub struct Config {
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
}

// Tree Generator
struct TreeGenerator {
    trees: Vec<String>
}

impl TreeGenerator {
    fn new() -> TreeGenerator {
        TreeGenerator {
            trees: vec![],
        }
    }

    fn get_trees(&self) -> &Vec<String> {
        &self.trees
    }

    fn build_tree(&mut self, root_dir: &Path) {
        self.tree_head(root_dir);
        self.tree_body(root_dir, "");
    }

    fn tree_head(&mut self, root_dir: &Path) {
        if let Some(file_name) = root_dir.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.trees.push(format!("{}{}", file_name, std::path::MAIN_SEPARATOR));
                self.trees.push(PIPE.to_string());
            }
        }
    }

    fn tree_body(&mut self, directory: &Path, prefix: &str) {
        let entries = self.prepare_entries(directory);
        let entries_count = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let connector = if index == entries_count-1 { ELBOW } else {TEE};
            if entry.is_dir() {
                self.add_directory(&entry, index, entries_count, prefix, connector);
            } else {
                self.add_file(&entry, prefix, connector);
            }
        }
    }

    fn prepare_entries(&mut self, directory: &Path) -> Vec<PathBuf> {
        directory.read_dir().unwrap().map(|dir| dir.unwrap().path()).collect()
    }

    fn add_directory(&mut self, directory: &Path, index: usize, entries_count: usize, prefix: &str, connector: &str) {
        self.trees.push(format!("{}{} {}{}", prefix, connector, directory.file_name().unwrap().to_str().unwrap(), std::path::MAIN_SEPARATOR));
        let prefix = if index != entries_count-1 {
            format!("{}{}", prefix, PIPE_PREFIX)
        } else {
            format!("{}{}", prefix, SPACE_PREFIX)
        };
        self.tree_body(directory, &prefix);
    }

    fn add_file(&mut self, file: &Path, prefix: &str, connector: &str) {
        self.trees.push(format!("{}{} {}", prefix, connector, file.file_name().unwrap().to_str().unwrap()));
    }
}


pub fn run(config: Config) {
    let mut tree_generator = TreeGenerator::new();
    tree_generator.build_tree(&config.root_dir);
    if let Some(_output_file) = config.output {
        todo!()
    } else {
        for entry in tree_generator.get_trees() {
            println!("{}", entry);
        }
    }

}