use std::{
    error, fs, io,
    path::{Path, PathBuf},
};

use crate::{TEMPLATES_DIR, WORKSPACE_DIR};

/// Get the path to the template for the specified name.
pub fn set_day(day_number: &u8) -> u8 {
    1
}

/// Get the name of the package for the given day.
pub fn set_day_prefix(prefix: &str) -> String {
    prefix.to_string()
}

pub fn set_job_name(){
    
}



pub fn write_input_data(path: &Path, data: &str) -> Result<(), io::Error> {
    // Ensure the parent paths exist or create them if necessary
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the input data to the input path; it's okay to overwrite
    let mut file = fs::File::create(path)?;
    io::Write::write_all(&mut file, data.as_bytes())?;
    Ok(())
}

pub fn initialize_day_package(
    package_name: &str,
    input_path: &Path,
    input_data: &str,
) -> Result<(), Box<dyn error::Error>> {
    let workspace_dir = WORKSPACE_DIR;
    let template_path = TEMPLATES_DIR.join("job");
    // let workspace_dir = crate::get_workspace_dir().unwrap();
    // let template_path = crate::get_template_path("day").unwrap();

    dbg!(workspace_dir.display());
    dbg!(template_path);
    dbg!(package_name);
    dbg!(input_path);
    // Run cargo generate
    // cargo generate --path ./templates/day --name {{day}}
    // let output = Command::new("cargo")
    //     .arg("generate")
    //     .arg("--path")
    //     .arg(template_path.to_str().unwrap()) // Convert Path to &str
    //     .arg("--name")
    //     .arg(package_name)
    //     .arg("--vcs")
    //     .arg("none") // Disable version control system initialization
    //     .output()?;

    // if !output.status.success() {
    //     eprintln!(
    //         "Error: Failed to run cargo generate: {}",
    //         String::from_utf8_lossy(&output.stderr)
    //     );
    //     exit(1);
    // }

    // // Write the input data to the input path
    // write_input_data(input_path, input_data)?;

    // println!("Package folder created successfully.");
    Ok(())
}
