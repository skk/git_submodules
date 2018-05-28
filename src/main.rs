#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate git2;
#[macro_use]
extern crate clap;
extern crate regex;

use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use regex::Regex;

use git2::Config;
use clap::{App, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct Submodule {
    name: String,
    path: String,
    url: String,
}

fn clone_repos(data_filename: &str) -> std::io::Result<()> {
    let mut data_file = File::open(data_filename)?;

    let mut contents = String::new();
    data_file.read_to_string(&mut contents).unwrap();
    let submodules: Vec<Box<Submodule>> = serde_json::from_str(&contents).unwrap();
    println!("{:?}", submodules);


    for submodule in submodules {
        println!("clone {} from {} to {}",
                 submodule.name, submodule.url, submodule.path);
    }

    Ok(())
}

fn generate_submodules_json_datafile(_repo: &git2::Repository,
                                     data_filename: &str) -> std::io::Result<()> {
    let mut data_file = File::create(data_filename)?;

    let mut submodules: Vec<Box<Submodule>> = Vec::new();

    let cfg = Config::open(Path::new("../.git/config")).unwrap();

    let entries = cfg.entries(None).unwrap();

    let re_entry_name = Regex::new(r"submodule\.(.*).url").unwrap();

    for entry in &entries {
        let entry = entry.unwrap();
        let name = entry.name().unwrap();
        let url = entry.value().unwrap();
        for cap in re_entry_name.captures_iter(name) {
            let mut name = cap[1].to_string();
            if name.starts_with(".") {
                let mut path = name.clone();
                name.remove(0);
                let s = Box::new(Submodule {
                    name: name.to_string(),
                    path: path.to_string(),
                    url: url.to_string(),
                });
                println!("submodule {:?}", s);
                submodules.push(s);
            }
        }
    }
    let serialized = serde_json::to_string(&submodules).unwrap();
    data_file.write_all(serialized.as_bytes())?;

    Ok(())
}

fn main() {
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

    let data_filename = "./repos.json";

    let repo = match git2::Repository::init("../") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    if matches.subcommand_matches("generate_json_file").is_some() {
        generate_submodules_json_datafile(&repo, &data_filename).unwrap();
    } else if matches.subcommand_matches("clone_repos").is_some() {
        clone_repos(&data_filename).unwrap();
    }
}

