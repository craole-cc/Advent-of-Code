use {
  crate::{
    AdminError,
    AoC,
    get_dotenv,
  },
  chrono::{
    Datelike,
    Local,
  },
  std::{
    fs::{
      File,
      create_dir_all,
    },
    io::Write,
    path::PathBuf,
  },
};

impl Default for AoC {
  fn default() -> Self {
    Self {
      token: Self::default_session_token(),
      year: Self::default_session_year(),
      day: 0,
    }
  }
}

impl AoC {
  /// Creates a new `AoC` with default values.
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_session_token(mut self, token: &str) -> Self {
    self.token = token.to_string();
    self
  }

  pub fn with_session_day(mut self, day: u8) -> Self {
    self.day = day;
    self
  }

  pub fn with_session_year(mut self, year: u16) -> Self {
    self.year = year;
    self
  }

  fn default_session_token() -> String {
    get_dotenv("AOC_SESSION_TOKEN").unwrap_or_default()
  }

  fn default_session_year() -> u16 {
    get_dotenv("AOC_SESSION_YEAR")
      .ok()
      .and_then(|value| value.parse().ok())
      .unwrap_or(Local::now().year() as u16)
  }

  fn get_year(&self) -> Result<(), AdminError> {
    let aoc_launch_year = 2015;
    let current_year = Local::now().year() as u16;

    if (aoc_launch_year..=current_year).contains(&self.year) {
      Ok(())
    } else {
      Err(AdminError::YearNotValid(self.year))
    }
  }

  fn get_day(&self) -> Result<(), AdminError> {
    if (1..=25).contains(&self.day) {
      Ok(())
    } else {
      Err(AdminError::DayNotValid(self.day))
    }
  }

  fn get_token(&self) -> Result<String, AdminError> {
    Ok(self.token.clone())
    // const EXPECTED_TOKEN_LENGTH: usize = 128;

    // if self.token.is_empty() {
    //     Err(AdminError::MissingSessionToken())
    // } else if self.token.len() != EXPECTED_TOKEN_LENGTH
    //     || !self.token.chars().all(|c| c.is_ascii_hexdigit())
    // {
    //     Err(AdminError::InvalidSessionToken(self.token.clone()))
    // } else {
    //     Ok(self.token.clone())
    // }
  }

  fn get_url(&self) -> Result<String, AdminError> {
    Ok(format!(
      "https://adventofcode.com/{}/day/{}/input",
      self.year, self.day
    ))
  }

  pub fn validate(&self) -> Result<(), AdminError> {
    self.get_year()?;
    self.get_day()?;
    self.get_token()?;
    Ok(())
  }

  pub fn fetch_data(&self) -> Result<String, AdminError> {
    self.validate()?;
    let url = self.get_url()?;
    let token = self.get_token()?;
    let client = reqwest::blocking::Client::new();

    let response = client
      .get(url)
      .header(reqwest::header::COOKIE, format!("session={}", token))
      .send()
      .map_err(AdminError::FailedReqwest)?
      .text()
      .map_err(AdminError::FailedReqwest)?;

    Ok(response)
  }

  pub fn deploy_data(&self, path: PathBuf) -> Result<(), AdminError> {
    let data = self.fetch_data()?;

    // Ensure the parent paths exist or create them if necessary
    if let Some(parent) = path.parent() {
      create_dir_all(parent).map_err(AdminError::FailedMkdir)?;
    }

    // Write the input data to the input path
    let mut file = File::create(&path).map_err(AdminError::FailedTouchFile)?;
    Write::write_all(&mut file, data.as_bytes()).map_err(AdminError::FailedWriteFile)?;
    Ok(())
  }
}
