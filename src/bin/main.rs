extern crate git_submodules;
use git_submodules::app::{App};
use git_submodules::arguments::parse_args;


fn main() {
    let matches = parse_args();
    // these should be CLI arguments, not hard-coded
    let mut _app = App::new("../dotfiles",
                            "./repos.json");

    if matches.subcommand_matches("generate_json_file").is_some() {
        _app.generate_submodules_json_datafile().unwrap();
    } else if matches.subcommand_matches("clone_repos").is_some() {
        _app.clone_repos().unwrap();
    }
}

