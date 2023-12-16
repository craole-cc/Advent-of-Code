use cargo_metadata::{Metadata, MetadataCommand};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
};

pub static METADATA: Lazy<Metadata> = Lazy::new(|| {
    MetadataCommand::new()
        .exec()
        .expect("Failed to get Cargo metadata")
});

pub static AOC_HOME: Lazy<PathBuf> = Lazy::new(init_env_aoc);
pub static WORKSPACE: Lazy<PathBuf> = Lazy::new(init_env_workspace);
pub static ASSETS_HOME: Lazy<PathBuf> = Lazy::new(init_env_assets);

fn init_env_workspace() -> PathBuf {
    env_metadata().workspace_root.into()
}

fn init_env_aoc() -> PathBuf {
    PathBuf::from(
        init_env_workspace()
            .parent()
            .expect("Failed to get parent")
            .parent()
            .expect("Failed to get parent"),
    )

    // let output = Command::new("git")
    //     .args(["rev-parse", "--show-toplevel"])
    //     .output()
    //     .expect("Failed to execute git command");

    // if output.status.success() {
    //     PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
    // } else {
    //     eprintln!("Error: Unable to determine AoC home directory.");
    //     exit(1);
    // }
}


fn init_env_assets() -> PathBuf {
    init_env_aoc().join("assets")
}

pub fn env_aoc() -> PathBuf {
    AOC_HOME.clone()
}

pub fn env_workspace() -> PathBuf {
    WORKSPACE.clone()
}

pub fn env_metadata() -> Metadata {
    METADATA.clone()
}

pub fn env_assets() -> PathBuf {
    env_aoc().join("assets")
}

// pub static METADATA: OnceCell<Metadata> = OnceCell::new();
// pub static AOC_HOME: OnceCell<PathBuf> = OnceCell::new();
// pub static WORKSPACE: OnceCell<PathBuf> = OnceCell::new();
// pub static ASSETS_HOME: OnceCell<PathBuf> = OnceCell::new();

// pub fn init_env() {
//     // init_metadata();
//     init_env_aoc();
//     init_env_workspace();
// }

// pub fn env_aoc() -> PathBuf {
//     AOC_HOME.get().unwrap().clone()
// }

// fn init_env_aoc() {
//     let output = Command::new("git")
//         .args(["rev-parse", "--show-toplevel"])
//         .output()
//         .expect("Failed to execute git command");

//     match output.status.success() {
//         true => AOC_HOME
//             .set(PathBuf::from(
//                 String::from_utf8_lossy(&output.stdout).trim().to_string(),
//             ))
//             .unwrap(),
//         false => {
//             eprintln!("Error: Unable to determine AoC home directory.");
//             exit(1);
//         }
//     }
// }

// pub fn env_workspace() -> PathBuf {
//     WORKSPACE.get().unwrap().clone()
// }

// pub fn env_metadata() -> Metadata {
//     // METADATA.get().unwrap().clone()
//     METADATA.clone()
// }

// // fn init_env_workspace() {
// //     WORKSPACE.set(env_metadata().workspace_root.into()).unwrap();
// // }
