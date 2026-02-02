use {
  crate::{
    AdminError,
    AoC,
    Package,
    get_workspace_dir,
  },
  std::{
    fs::{
      File,
      create_dir_all,
    },
    io::Write,
    path::PathBuf,
    process::{
      Command,
      ExitStatus,
      Output,
      Stdio,
    },
  },
};

impl Default for Package {
  /// Creates a new `Package` with default values.
  fn default() -> Self {
    Self {
      base_name: String::from("package"),
      number: None,
      digits: None,
      is_aoc: false,
      aoc: AoC::default(),
    }
  }
}

impl Package {
  /// Creates a new `Package` with default values.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the base name for the formatter.
  pub fn with_name(mut self, base_name: &str) -> Self {
    self.base_name = base_name.to_string();
    self
  }

  /// Sets the incremental number for the formatter.
  pub fn with_sequence_number(mut self, number: u8) -> Self {
    self.number = Some(number);
    self.aoc.day = number;
    self
  }

  /// Sets the padding width for the incremental number.
  pub fn with_digits(mut self, digits: u8) -> Self {
    self.digits = Some(digits);
    self
  }

  /// Sets whether the package is an AoC package.
  pub fn as_aoc(mut self) -> Self {
    self.is_aoc = true;
    self
  }

  pub fn with_aoc_year(mut self, year: u16) -> Self {
    self.aoc.year = year;
    self.is_aoc = true;
    self
  }

  pub fn with_aoc_day(mut self, day: u8) -> Self {
    self.aoc.day = day;
    self.number = Some(day);
    self.is_aoc = true;
    self
  }

  pub fn with_aoc_token(mut self, token: &str) -> Self {
    self.aoc.token = token.to_string();
    self.is_aoc = true;
    self
  }

  fn path(&self) -> PathBuf {
    let workspace_dir = get_workspace_dir();
    workspace_dir.join(self.to_string())
  }

  fn name(&self) -> String {
    self.to_string()
  }

  pub fn display(&self) -> String {
    self.to_string()
  }

  fn aoc_input_path(&self) -> PathBuf {
    self.path().join("assets").join("input.txt")
  }

  pub fn deploy(&self) -> Result<(), AdminError> {
    if self.is_aoc {
      self.aoc.validate()?
    }

    let cmd_cargo_update = Command::new("cargo")
      .arg("update")
      .arg(&self.name())
      .stdout(Stdio::inherit())
      .stderr(Stdio::null()) // Redirect stderr to null
      .status();

    if let Ok(update_status) = cmd_cargo_update {
      if update_status.success() {
        self.aoc.deploy_data(self.aoc_input_path())?;
        Ok(())
      } else {
        let cmd_cargo_new = Command::new("cargo")
          .arg("new")
          .arg(&self.name())
          .arg("--vcs")
          .arg("none")
          .output(); // Using output() instead of status() to get the stderr

        match cmd_cargo_new {
          Ok(output) if output.status.success() => {
            self.aoc.deploy_data(self.aoc_input_path())?;
            Ok(())
          }
          _ => {
            let stderr = String::from_utf8_lossy(&cmd_cargo_new.as_ref().unwrap().stderr);
            let status = cmd_cargo_new.as_ref().unwrap().status;

            Err(AdminError::NonZeroExit(
              format!("'cargo new {}'", &self.name()),
              status,
              stderr.into(),
            ))
          }
        }
      }
    } else {
      Err(AdminError::FailedCommandExecution(
        format!("'cargo update {}'", &self.name()),
        cmd_cargo_update.unwrap_err(),
      ))
    }
  }
}

/// Formats the incremental package name when used in a `String` context.
impl ToString for Package {
  /// Converts the `Package` into a formatted string.
  fn to_string(&self) -> String {
    let number = self.number.unwrap_or_default();
    let digits = self.digits.unwrap_or_default() as usize;

    format!(
      "{}-{number:0width$}",
      self.base_name,
      number = number,
      width = digits
    )
  }
}
