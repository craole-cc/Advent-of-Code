use {
  crate::GlobalEnvironment,
  cargo_metadata::{
    Metadata,
    MetadataCommand,
  },
  std::path::PathBuf,
};

impl Default for GlobalEnvironment {
  fn default() -> Self {
    Self {
      cargo_meta: get_metadata(),
      workspace: PathBuf::new(),
      aoc_home: PathBuf::new(),
      aoc_session_token: String::new(),
      establish_environment: (),
    }
  }
}

impl GlobalEnvironment {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get_workspace_path(mut self) -> Self {
    let path = get_env_value("CARGO_MANIFEST_DIR")
      .ok()
      .map(|manifest_dir| {
        PathBuf::from(manifest_dir)
          .ancestors()
          .nth(3)
          .expect("Failed to find workspace path")
          .to_path_buf()
      })
      .or_else(|| Some(PathBuf::from(&self.cargo_meta.workspace_root)))
      .expect("Failed to determine workspace path");

    self.workspace = path;
    self
  }
}

fn get_metadata() -> Metadata {
  MetadataCommand::new()
    .exec()
    .expect("Failed to get Cargo metadata")
}

pub fn env_init_from(directory: &std::path::Path) {
  fn is_env_file(entry: &walkdir::DirEntry) -> bool {
    entry
      .file_name()
      .to_str()
      .map(|s| s.ends_with(".env"))
      .unwrap_or(false)
  }

  for entry in walkdir::WalkDir::new(directory)
    .into_iter()
    .filter(|entry| entry.is_ok() && is_env_file(entry.as_ref().unwrap()))
    .filter_map(|entry| entry.ok())
  {
    dotenv_rs::from_filename(entry.path()).ok();
  }
}

pub fn get_env_value(key: &str) -> Result<String, crate::AdminError> {
  dotenv_rs::var(key).map_err(|_| crate::AdminError::variable_not_set(key))
}
