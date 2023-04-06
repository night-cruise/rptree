use structopt::StructOpt;

use rptree::opt::Config;

fn main() {
    let config = Config::from_args();
    if !config.is_root_dir() {
        eprintln!("ERROR: The specified root directory doesn't exist.");
        std::process::exit(1);
    }

    if let Some(color) = config.get_color() {
        if color != "green" && color != "blue" && color != "red" {
            eprintln!("ERROR: Only supports green, blue and red color.");
            std::process::exit(1);
        }
    }

    if let Err(err) = rptree::run(config) {
        eprintln!("ERROR: {err}");
        if let Some(source) = err.source() {
            eprintln!("SOURCE: {source}");
        }
        std::process::exit(1);
    }
}
