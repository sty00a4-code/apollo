extern crate clap;

use std::{env, error::Error, fmt::Display, fs, path::Path, process};

use clap::{Parser, Subcommand};
use toml::{map::Map, Table, Value};

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
        PackageCommand::Add { name } => {
            add(&name)
                .map_err(|err| {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                })
                .unwrap();
        }
        PackageCommand::Remove { name } => {
            remove(&name)
                .map_err(|err| {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                })
                .unwrap();
        }
        PackageCommand::Update => todo!(),
    }
}

pub fn new(name: &str, path: Option<String>) -> Result<(), InitError> {
    let package_file_path = if let Some(path) = &path {
        fs::create_dir(path).map_err(InitError::CreatingDirectory)?;
        path.clone() + "/apollo.toml"
    } else {
        "apollo.toml".to_string()
    };
    let lib_file_path = if let Some(path) = &path {
        fs::create_dir_all(&path).map_err(InitError::CreatingDirectory)?;
        path.clone() + "/init.luna"
    } else {
        "init.luna".to_string()
    };
    let contents = toml::toml! {
        [package]
        name = name
        version = "v0.1"
        lib = "init.luna"
        [dependencies]
    };
    if fs::read(&package_file_path).is_ok() {
        return Err(InitError::AlreadyExists);
    }
    fs::write(&package_file_path, contents.to_string()).map_err(InitError::WritingFile)?;
    if !fs::read(&lib_file_path).is_ok() {
        fs::write(&lib_file_path, "return {}").map_err(InitError::WritingFile)?;
    }
    Ok(())
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
            Self::AlreadyExists => write!(f, "package already present"),
            Self::WritingFile(err) => {
                write!(f, "can not write to the `apollo.toml` file: {err}")
            }
            Self::CreatingDirectory(err) => {
                write!(f, "can not create directory: {err}")
            }
        }
    }
}
impl Error for InitError {}

pub fn add(name: &str) -> Result<(), DependencyError> {
    let content = fs::read_to_string("apollo.toml").map_err(|_| DependencyError::NoPackage)?;
    let mut table = toml::from_str::<Table>(&content).map_err(DependencyError::ParsingTOML)?;
    if !table.contains_key("dependencies") {
        table.insert("dependencies".to_string(), Value::Table(Map::default()));
    }
    let Value::Table(ref mut dependencies) = table.get_mut("dependencies").unwrap() else {
        return Err(DependencyError::DependenciesNotTable);
    };
    let version = String::from("*");
    dependencies.insert(name.to_string(), Value::String(version));
    fs::write("apollo.toml", table.to_string()).map_err(DependencyError::WritingFile)?;
    Ok(())
}
pub fn remove(name: &str) -> Result<(), DependencyError> {
    let content = fs::read_to_string("apollo.toml").map_err(|_| DependencyError::NoPackage)?;
    let mut table = toml::from_str::<Table>(&content).map_err(DependencyError::ParsingTOML)?;
    if !table.contains_key("dependencies") {
        table.insert("dependencies".to_string(), Value::Table(Map::default()));
    }
    let Value::Table(ref mut dependencies) = table.get_mut("dependencies").unwrap() else {
        return Err(DependencyError::DependenciesNotTable);
    };
    dependencies.remove(name);
    fs::write("apollo.toml", table.to_string()).map_err(DependencyError::WritingFile)?;
    Ok(())
}
#[derive(Debug)]
pub enum DependencyError {
    NoPackage,
    WritingFile(std::io::Error),
    ParsingTOML(toml::de::Error),
    DependenciesNotTable,
}
impl Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPackage => write!(f, "no package in this directory"),
            Self::WritingFile(err) => {
                write!(f, "can not write to the `apollo.toml` file: {err}")
            }
            Self::ParsingTOML(err) => write!(f, "{err}"),
            Self::DependenciesNotTable => write!(f, "`dependencies` key is not a table"),
        }
    }
}
impl Error for DependencyError {}
