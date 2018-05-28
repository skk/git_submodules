use clap::{App, SubCommand, ArgMatches};


pub fn parse_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("git_submodules")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("generate_json_file")
            .about("parse git submodules to json file")
        )
        .subcommand(SubCommand::with_name("clone_repos")
            .about("clone any missing repos")
        )
        .get_matches();
    matches
}
