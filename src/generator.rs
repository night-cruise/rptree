use std::io;
use std::path::{Path, PathBuf};

const PIPE: &str = "|";
const ELBOW: &str = "└──";
const TEE: &str = "├──";
const PIPE_PREFIX: &str = "│   ";
const SPACE_PREFIX: &str = "    ";

/// Tree Generator, for generating the directory trees.
#[derive(Debug)]
pub struct TreeGenerator {
    dir_only: bool,
    trees: Vec<String>,
}

impl TreeGenerator {
    pub fn new(dir_only: bool) -> TreeGenerator {
        TreeGenerator {
            dir_only,
            trees: vec![],
        }
    }

    /// Get the directory trees result.
    pub fn get_trees(&self) -> &[String] {
        &self.trees
    }

    /// Build the directory trees.
    pub fn build_tree(&mut self, root_dir: &Path, filter: &[String]) -> io::Result<()> {
        self.tree_head(root_dir);
        self.tree_body(root_dir, "", filter)?;

        Ok(())
    }

    /// Parse tree head.
    fn tree_head(&mut self, root_dir: &Path) {
        if let Some(file_name) = root_dir.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.trees
                    .push(format!("{}{}", file_name, std::path::MAIN_SEPARATOR));
                self.trees.push(PIPE.to_string());
            }
        }
    }

    /// Parse tree body.
    fn tree_body(&mut self, directory: &Path, prefix: &str, filter: &[String]) -> io::Result<()> {
        let entries = self.prepare_entries(directory, filter)?;
        let entries_count = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let connector = if index == entries_count - 1 {
                ELBOW
            } else {
                TEE
            };
            if entry.is_dir() {
                self.add_directory(entry, index, entries_count, prefix, connector, filter)?;
            } else {
                self.add_file(entry, prefix, connector);
            }
        }

        Ok(())
    }

    fn prepare_entries(&mut self, directory: &Path, filter: &[String]) -> io::Result<Vec<PathBuf>> {
        let mut v = vec![];
        let dirs = directory.read_dir()?;

        for dir in dirs {
            let path = dir?.path();
            if !filter.contains(&path.file_name().unwrap().to_string_lossy().into_owned()) {
                if self.dir_only {
                    if path.is_dir() {
                        v.push(path);
                    }
                } else {
                    v.push(path);
                }
            }
        }

        if !self.dir_only {
            v.sort_by_key(|a| a.is_file());
        }

        Ok(v)
    }

    fn add_directory(
        &mut self,
        directory: &Path,
        index: usize,
        entries_count: usize,
        prefix: &str,
        connector: &str,
        filter: &[String],
    ) -> io::Result<()> {
        if let Some(file_name) = directory.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.trees.push(format!(
                    "{}{} {}{}",
                    prefix,
                    connector,
                    file_name,
                    std::path::MAIN_SEPARATOR
                ));

                let prefix = if index != entries_count - 1 {
                    prefix.to_owned() + PIPE_PREFIX
                } else {
                    prefix.to_owned() + SPACE_PREFIX
                };
                self.tree_body(directory, &prefix, filter)?;
                self.trees.push(prefix.trim_end().to_string());
            }
        }
        Ok(())
    }

    fn add_file(&mut self, file: &Path, prefix: &str, connector: &str) {
        if let Some(file_name) = file.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.trees.push(format!("{prefix}{connector}{file_name}"));
            }
        }
    }
}
