use std::fs::File;
use std::io;

use colored::Colorize;
use generator::TreeGenerator;
use opt::Config;

pub mod generator;
pub mod opt;

/// Run the program.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree_generator = TreeGenerator::new(config.dir_only());
    tree_generator.build_tree(config.get_root_dir(), config.get_filter())?;

    if let Some(file_name) = config.get_output_file() {
        output_to_file(File::create(file_name)?, tree_generator.get_trees())?;
    } else {
        output_to_stdout(io::stdout(), tree_generator.get_trees(), config.get_color())?;
    }

    Ok(())
}

/// Write output to stdout.
fn output_to_stdout(
    mut writer: impl io::Write,
    trees: &[String],
    color: Option<&str>,
) -> io::Result<()> {
    if let Some(color) = color {
        match color {
            "green" => {
                for entry in trees {
                    writeln!(writer, "{}", entry.green().bold())?;
                }
            }
            "blue" => {
                for entry in trees {
                    writeln!(writer, "{}", entry.blue().bold())?;
                }
            }
            "red" => {
                for entry in trees {
                    writeln!(writer, "{}", entry.red().bold())?;
                }
            }
            _ => panic!("ERROR: Unsupported color."),
        }
    } else {
        for entry in trees {
            writeln!(writer, "{entry}")?;
        }
    }

    Ok(())
}

/// Write output to file.
fn output_to_file(mut writer: impl io::Write, trees: &[String]) -> io::Result<()> {
    for entry in trees {
        writeln!(writer, "{entry}")?;
    }

    Ok(())
}
