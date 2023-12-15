use crate::{get_dotenv, validate_and_format_year, AdminError};
use chrono::{Datelike, Local};
use lazy_static::lazy_static;
use std::{path::PathBuf, process::exit};

lazy_static! {
    pub static ref WORKSPACE_DIR: PathBuf = get_workspace_dir();
    pub static ref TEMPLATES_DIR: PathBuf = WORKSPACE_DIR.join("templates");
    pub static ref AOC_SESSION_KEY: String = get_session_key();
    pub static ref AOC_SESSION_YEAR: String = get_session_year();
}

pub fn hello_from_env() {
    println!("Hello from Env!");
    println!("Workspace Directory: {:?}", *WORKSPACE_DIR);
    println!("Templates Directory: {:?}", *TEMPLATES_DIR);
    println!("Session Key: {:?}", *AOC_SESSION_KEY);
    println!("Session Year: {:?}", *AOC_SESSION_YEAR);
    println!();
}

pub fn get_workspace_dir() -> PathBuf {
    let manifest_dir = match get_dotenv("CARGO_MANIFEST_DIR") {
        Ok(value) => PathBuf::from(value),
        Err(err) => {
            eprintln!("{}\n{}", &err, AdminError::dir_not_found("workspace"));

            // Fallback to the current directory
            return env::current_dir().expect("Failed to get current directory");
        }
    };

    match manifest_dir.parent() {
        Some(workspace_dir) => workspace_dir.to_path_buf(),
        None => {
            eprintln!("{}", AdminError::dir_not_found("workspace"));

            // Fallback to the current directory
            env::current_dir().expect("Failed to get current directory")
        }
    }
}

/// Get the session key from the environment.
pub fn get_session_key() -> String {
    match get_dotenv("AOC_SESSION_KEY") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}\n{}", &err, AdminError::MissingSessionKey());
            exit(1);
        }
    }
}

/// Get the year of the Advent of Code.
pub fn get_session_year() -> String {
    let aoc_launch_year = 2015;
    let current_year = Local::now().year() as u16;

    let mut year = current_year;

    if let Ok(env_year) = get_dotenv("AOC_SESSION_YEAR") {
        year = env_year.parse().unwrap_or(current_year);
    } else if let Some(parent) = WORKSPACE_DIR.parent() {
        if let Some(path_year) = parent.file_name().and_then(|file_name| file_name.to_str()) {
            year = path_year.parse().unwrap_or(current_year);
        }
    }

    match validate_and_format_year(year, aoc_launch_year, current_year) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
