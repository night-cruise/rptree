use structopt::StructOpt;

use rptree::opt::Config;

fn main() {
    let config = Config::from_args();
    if !config.is_root_dir() {
        eprintln!("Error: The specified root directory doesn't exist");
        std::process::exit(1);
    }

    if let Err(err) = rptree::run(config) {
        eprintln!("Error: {}", err);
        if let Some(cause) = err.source() {
            eprintln!("Cause: {}", cause);
        }
        std::process::exit(1);
    }
}
