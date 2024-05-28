extern crate clap;

use std::{env, error::Error, fmt::Display, fs, path::Path, process};

use clap::{Parser, Subcommand};

pub const REPO_LINK: &str = "https://raw.githubusercontent.com/sty00a4-code/apollo/master";
pub fn package_file_link(package: &str) -> String {
    format!("{REPO_LINK}/registry/{package}")
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: PackageCommand,
}
#[derive(Debug, Subcommand, Clone)]
pub enum PackageCommand {
    Init { name: Option<String> },
    New { name: String },
    Add { name: String },
    Remove { name: String },
    Update,
}

fn main() {
    let args = Args::parse();
    match args.command {
        PackageCommand::Init { name } => {
            let name = if let Some(name) = name {
                name
            } else {
                let path = env::current_dir()
                    .map_err(|err| {
                        eprintln!("{err}");
                        process::exit(1);
                    })
                    .unwrap();
                if let Some(path) = path.file_name().and_then(|s| s.to_str()) {
                    path.to_string()
                } else {
                    eprintln!("ERROR: couldn't get name of the directory");
                    process::exit(1);
                }
            };
            new(&name, None)
                .map_err(|err| {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                })
                .unwrap();
        }
        PackageCommand::New { name } => {
            new(&name, Some(name.clone()))
                .map_err(|err| {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                })
                .unwrap();
        }
        PackageCommand::Add { name } => todo!(),
        PackageCommand::Remove { name } => todo!(),
        PackageCommand::Update => todo!(),
    }
}

pub fn new(name: &str, path: Option<String>) -> Result<(), InitError> {
    let file_path = if let Some(path) = path {
        fs::create_dir(&path).map_err(InitError::CreatingDirectory)?;
        path + "/apollo.toml"
    } else {
        "apollo.toml".to_string()
    };
    let contents = toml::toml! {
        [package]
        name = name
        version = "v0.1"
        lib = "init.luna"
        [dependencies]
    };
    if fs::read(&file_path).is_ok() {
        return Err(InitError::AlreadyExists);
    }
    fs::write(&file_path, contents.to_string()).map_err(InitError::WritingFile)
}
#[derive(Debug)]
pub enum InitError {
    AlreadyExists,
    WritingFile(std::io::Error),
    CreatingDirectory(std::io::Error),
}
impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::AlreadyExists => write!(f, "package already present"),
            InitError::WritingFile(err) => {
                write!(f, "can not write to the `apollo.toml` file: {err}")
            }
            InitError::CreatingDirectory(err) => {
                write!(f, "can not create directory: {err}")
            }
        }
    }
}
impl Error for InitError {}
