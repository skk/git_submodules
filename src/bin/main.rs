extern crate git_submodules;
extern crate flexi_logger;

use git_submodules::app::App;
use git_submodules::arguments::parse_args;

use flexi_logger::{detailed_format};
use flexi_logger::Logger;

fn setup_logging(logging_level: &str) {
    Logger::with_str(logging_level)
        .format(detailed_format)
        .print_message()
        .suppress_timestamp()
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}

fn main() {
    let matches = parse_args();
    setup_logging("info");

    // these should be CLI arguments, not hard-coded
    let mut _app = App::new("../../dotfiles",
                            "./repos.json");

    if matches.subcommand_matches("generate_json_file").is_some() {
        _app.generate_submodules_json_datafile().unwrap();
    } else if matches.subcommand_matches("clone_repos").is_some() {
        _app.clone_repos().unwrap();
    }
}

