use cargo_metadata::{Metadata, MetadataCommand};
use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    process::{exit, Command},
};

pub static METADATA: Lazy<Metadata> = Lazy::new(|| {
    MetadataCommand::new()
        .exec()
        .unwrap_or_else(|e| panic!("Failed to get Cargo metadata: {}", e))
});

pub fn env_workspace() -> PathBuf {
    METADATA.clone().workspace_root.into()
}

pub fn env_aoc() -> PathBuf {
    PathBuf::from(
        env_workspace()
            .parent()
            .expect("Failed to get parent")
            .parent()
            .expect("Failed to get parent"),
    )
}

pub fn env_assets() -> PathBuf {
    env_aoc().join("assets")
}
