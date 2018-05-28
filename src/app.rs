use std::path::Path;
use std::io::Write;
use std::io::Result;
use std::io::Read;
use std::fs::OpenOptions;
use std::fs::File;
use regex::Regex;

use git2::{Repository, Config};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Submodule {
    name: String,
    path: String,
    url: String,
}

// `git2::Repository` cannot be formatted using `:?` because it doesn't implement `std::fmt::Debug`
// #[derive(Debug)]
pub struct App {
    repo: Repository,
    gitmodules_datafile: File,
}


impl App {
    pub fn new(repo_path: &str, data_filename: &str) -> App {
        let repo = match Repository::init(repo_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        let gitmodules_datafile = match OpenOptions::new().
            read(true).
            write(true).
            open(data_filename) {
                Ok(datafile) => datafile,
                Err(e) => panic!("failed to init: {}", e),
        };

        App {
            repo,
            gitmodules_datafile,
        }
    }
    pub fn clone_repos(&mut self) -> Result<()> {
        let mut contents = String::new();
        self.gitmodules_datafile.read_to_string(&mut contents).unwrap();
        let submodules: Vec<Box<Submodule>> = serde_json::from_str(&contents).unwrap();
        println!("{:?}", submodules);


        for submodule in submodules {
            println!("clone {} from {} to {}",
                     submodule.name, submodule.url, submodule.path);
        }

        Ok(())
    }

    pub fn generate_submodules_json_datafile(&mut self) -> Result<()> {
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
        self.gitmodules_datafile.write_all(serialized.as_bytes())?;

        Ok(())
    }
}
