extern crate git_submodules;
extern crate flexi_logger;

#[macro_use]
extern crate log;

use std::process::{exit};

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
    let repo_datafile = matches.value_of("datafile").unwrap_or("./repos.json");
    let repo_paths = matches.values_of("repo_paths");
    let debug = matches.is_present("debug");

    let log_level = match debug {
        true => "debug",
        _ => "info"
    };
    setup_logging(log_level);

    if repo_paths.is_none() {
        warn!("No repo paths provided");
        exit(1);
    }
    let repos: Vec<&str> = repo_paths.unwrap().collect();

    // TODO support multiple repos
    let repo = repos.iter().nth(0).unwrap();

    info!("repo {:?}", repo);
    debug!("matches {:?}", matches);
    debug!("log_level {}", log_level);
    debug!("debug {}", debug);
    debug!("repo_datafile {}", repo_datafile);
    debug!("repos {:?}", repos);

    let mut _app = App::new(repo,
                            repo_datafile);

    if matches.subcommand_matches("generate_json_file").is_some() {
        _app.generate_submodules_json_datafile().unwrap();
    } else if matches.subcommand_matches("clone_repos").is_some() {
        _app.clone_repos().unwrap();
    }
}

