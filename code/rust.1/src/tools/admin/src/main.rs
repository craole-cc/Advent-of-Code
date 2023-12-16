use std::path::{Path, PathBuf};

use admin::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init_env();
    dbg!(env_aoc(), env_workspace(), env_assets());

    Ok(())
}

fn test_package_init() {
    match Package::new()
        .with_name("day")
        .with_digits(2)
        .with_aoc_day(60)
        .deploy()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
}
