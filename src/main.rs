use rptree::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    if !config.is_root_dir() {
        eprintln!("Error: The specified root directory doesn't exist");
        std::process::exit(1);
    }

    rptree::run(config);
}