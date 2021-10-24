use std::fs::File;
use std::io;

use generator::TreeGenerator;
use opt::Config;

mod generator;
pub mod opt;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree_generator = TreeGenerator::new();
    tree_generator.build_tree(config.get_root_dir())?;

    if let Some(file_name) = config.get_output_file() {
        write_output(File::create(file_name)?, tree_generator.get_trees())?;
    } else {
        write_output(io::stdout(), tree_generator.get_trees())?;
    }

    Ok(())
}

fn write_output(mut writer: impl io::Write, trees: &[String]) -> io::Result<()> {
    for entry in trees {
        writeln!(writer, "{}", entry)?;
    }

    Ok(())
}
