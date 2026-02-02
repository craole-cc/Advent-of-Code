use {
  crate::{
    AOC_HOME,
    AdminError,
    CARGO_META,
    ESTABLISH_ENVIRONMENT,
    WORKSPACE,
    env_utils::{
      env_init_from,
      get_env_value,
    },
  },
  dotenv_rs::var,
  once_cell::sync::Lazy,
  std::{
    path::PathBuf,
    process::exit,
  },
};

pub fn init_env() {
  env_init_from(&AOC_HOME);
}

pub fn get_workspace_dir() -> PathBuf {
  var("CARGO_MANIFEST_DIR")
    .ok()
    .map(|manifest_dir| {
      PathBuf::from(manifest_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to find workspace path")
        .to_path_buf()
    })
    .or_else(|| {
      let metadata = Lazy::force(&CARGO_META);
      Some(metadata.workspace_root.as_path().into())
    })
    .expect("Failed to determine workspace path")
}

pub fn get_aoc_dir() -> PathBuf {
  WORKSPACE
    .parent()
    .expect("Failed to get parent")
    .parent()
    .expect("Failed to get parent")
    .to_path_buf()
}

pub fn get_aoc_token() -> String {
  // Initialize the environment from the .env files, if they haven't been initialized yet.
  Lazy::force(&ESTABLISH_ENVIRONMENT);

  let expected_length: usize = 128;
  let key: &str = "AOC_SESSION_TOKENs";

  let token = match get_env_value(key) {
    Ok(value) => {
      if value.is_empty() {
        Err(AdminError::unset_session_token(key))
      } else if value.len() != expected_length || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        Err(AdminError::InvalidSessionToken(value))
      } else {
        Ok(value)
      }
    }
    Err(e) => Err(AdminError::unset_session_token(key)),
  };

  match token {
    Ok(token) => token,
    Err(e) => {
      AdminError::print(&e);
      exit(1);
    }
  }
}
