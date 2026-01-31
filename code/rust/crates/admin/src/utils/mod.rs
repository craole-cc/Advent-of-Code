pub mod print;

pub use print::*;

use std::{
    env,
    path::{Path, PathBuf}, process,
};

use crate::AdminError;

/// Get environment variable using `dotenv`.
pub fn get_dotenv(key: &str) -> Result<String, AdminError> {
    env::var(key).map_err(|_| AdminError::variable_not_set(key))
    // dotenv::var(key).map_err(|_| AdminError::variable_not_set(key))
}

/// Get the path to the workspace directory.
pub fn get_workspace_dir() -> PathBuf {
    let manifest_dir = match get_dotenv("CARGO_MANIFEST_DIR") {
        Ok(value) => PathBuf::from(value),
        Err(err) => {
            eprintln!("{}\n{}", &err, AdminError::dir_not_found("workspace"));

            // Fallback to the current directory
            env::current_dir().expect("Failed to get current directory")
        }
    };

    match manifest_dir.parent() {
        Some(workspace_dir) => workspace_dir.to_path_buf(),
        None => {
            eprintln!("{}", AdminError::dir_not_found("workspace"));

            // Exit the program if the workspace directory cannot be determined
            process::exit(1)
        }
    }
}

/// Get the path to the template for the specified name.
pub fn get_package_path(package_name: &str) -> Result<PathBuf, AdminError> {
    Ok(get_workspace_dir().join(package_name))
}

/// Validate and format the year.
pub fn validate_and_format_year(
    year: u16,
    launch_year: u16,
    current_year: u16,
) -> Result<String, AdminError> {
    match year {
        year if (launch_year..=current_year).contains(&year) => Ok(year.to_string()),
        _ => Err(AdminError::YearNotValid(year)),
    }
}

/// Formats an incremental package name with leading zeros for the given day.
///
/// This function takes a base name, a number, and a padding value, and returns a formatted
/// string with the incremental number padded to the specified width with leading zeros.
///
/// # Arguments
///
/// * `name` - The base name of the package.
/// * `number` - The incremental number to be formatted.
/// * `padding` - The width of the padding for the incremental number.
///
/// # Returns
///
/// A formatted string containing the incremental package name.
///
/// # Examples
///
/// ```rust
/// let result = format_incremental_package_name("day", 1, 3);
/// assert_eq!(result, "day-001");
/// ```
pub fn format_incremental_package_name(name: &str, number: u8, digits: u8) -> String {
    format!(
        "{}-{number:0width$}",
        name,
        number = number,
        width = digits as usize
    )
}

/// Retrieve the path to a child directory, or error if it does not exist.
pub fn get_abs_path<P: AsRef<Path>>(parent: P, child: &str) -> PathBuf {
    parent.as_ref().join(child)
}

/// Retrieve the path to a child directory, or error if it does not exist.
pub fn validate_child_path<P: AsRef<Path>>(parent: P, child: &str) -> Result<PathBuf, AdminError> {
    let child_path = get_abs_path(parent, child);

    if child_path.exists() {
        Ok(child_path)
    } else {
        eprintln!("{}", AdminError::dir_not_found(child));
        Err(AdminError::dir_not_found(child))
    }
}
