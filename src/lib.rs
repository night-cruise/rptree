use std::fs::File;
use std::io;

use generator::TreeGenerator;
use opt::Config;

pub mod generator;
pub mod opt;

/// Run the program.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree_generator = TreeGenerator::new(config.dir_only());
    tree_generator.build_tree(config.get_root_dir())?;

    if let Some(file_name) = config.get_output_file() {
        write_output(File::create(file_name)?, tree_generator.get_trees())?;
    } else {
        write_output(io::stdout(), tree_generator.get_trees())?;
    }

    Ok(())
}

/// Write output to stdout or file.
fn write_output(mut writer: impl io::Write, trees: &[String]) -> io::Result<()> {
    for entry in trees {
        writeln!(writer, "{}", entry)?;
    }

    Ok(())
}
