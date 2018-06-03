use clap::{App, Arg, SubCommand, ArgMatches};

// https://github.com/kbknapp/clap-rs/blob/master/examples/20_subcommands.rs
pub fn parse_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("git_submodules")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("debug")
            .help("turn on debugging information")
            .long("debug")
            .short("d"))
        .arg(Arg::with_name("repo_paths")
            .short("r")
            .takes_value(true)
            .multiple(true)
            .long("repos")
            .help("location of repo(s)"))
        .arg(Arg::with_name("repo_datafile")
            .short("f")
            .takes_value(true)
            .multiple(false)
            .long("repo_datafile")
            .help("generated json datafile name"))
        .subcommand(SubCommand::with_name("generate_json_file")
            .about("parse git submodules to json file")
        )
        .subcommand(SubCommand::with_name("clone_repos")
            .about("clone any missing repos")
        )
        .get_matches();
    matches
}
