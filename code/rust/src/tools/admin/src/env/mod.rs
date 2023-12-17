pub mod env_utils;
pub mod local;

use cargo_metadata::Metadata;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static CARGO_META: Lazy<Metadata> = Lazy::new(env_utils::get_metadata);
pub static WORKSPACE: Lazy<PathBuf> = Lazy::new(local::get_workspace_dir);
pub static AOC_HOME: Lazy<PathBuf> = Lazy::new(local::get_aoc_dir);
pub static ESTABLISH_ENVIRONMENT: Lazy<()> = Lazy::new(local::init_env);
pub static AOC_SESSION_TOKEN: Lazy<String> = Lazy::new(local::get_aoc_token);

pub fn hello_from_env() {
    let crate_name = "env";
    println!("Hello, from the {} crate!", crate_name);
    // println!("Workspace: {:?}", &**WORKSPACE);
    // println!();

    // println!();
    // println!("AoC Session Token: {:?}", &**AOC_SESSION_TOKEN);
}
