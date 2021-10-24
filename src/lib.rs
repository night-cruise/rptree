use std::fs::File;
use std::io;
use std::path::PathBuf;

use generator::TreeGenerator;
use opt::Config;

mod generator;
pub mod opt;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree_generator = TreeGenerator::new();
    tree_generator.build_tree(config.get_root_dir())?;

    let writer = get_writer(config.get_output_file())?;
    write_output(writer, tree_generator.get_trees())?;

    Ok(())
}

fn get_writer(output_file: &Option<PathBuf>) -> io::Result<Box<dyn io::Write>> {
    let writer: Box<dyn io::Write> = if let Some(output_file) = output_file {
        Box::new(File::create(output_file)?)
    } else {
        Box::new(io::stdout())
    };

    Ok(writer)
}

fn write_output(mut writer: impl io::Write, trees: &[String]) -> io::Result<()> {
    for entry in trees {
        writeln!(writer, "{}", entry)?;
    }

    Ok(())
}
