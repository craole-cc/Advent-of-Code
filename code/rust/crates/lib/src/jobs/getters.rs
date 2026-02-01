use std::{
    error,
    io,
    path::{Path, PathBuf}, fs,
};

use crate::{WORKSPACE_DIR,TEMPLATES_DIR};

/// Get the path to the template for the specified name.
pub fn get_day_path(package_name: &str) -> Result<PathBuf, Box<dyn error::Error>> {
    Ok(crate::utils::get_package_path(package_name)?)
}

/// Get the name of the package for the given day.
pub fn get_day_name(day: &u8) -> String {
    format!("day-{:02}", day)
    
}

/// Get the path to the input file for the given day.
pub fn get_input_path(package_name: &Path) -> Result<PathBuf, Box<dyn error::Error>> {
    Ok(get_day_path(package_name)?.join("assets").join("input.txt"))
}

/// Get the URL for the Advent of Code input for the given day.
pub fn get_input_url(year: &u16, day: &u8) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

/// Get the input data from the provided URL.
pub fn get_input_data(url: &str, session_key: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session_key))
        .send()?
        .text()?;

    Ok(response)
}