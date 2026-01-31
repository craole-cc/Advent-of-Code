use std::{
    borrow::Cow,
    error::{self, Error},
    io,
    process::ExitStatus,
};

#[derive(thiserror::Error, Debug)]
pub enum AdminError {
    #[error("Failed to determine the directory location for the {0}. Please check your configuration and try again.")]
    DirectoryNotLocated(Cow<'static, str>),

    #[error("Environment Error: Failed to retrieve a value from the key '{0}'.")]
    VariableNotSet(Cow<'static, str>),

    #[error("The session token {0} is invalid. Please refer to the README for more information.")]
    InvalidSessionToken(String),

    #[error(
            "The session token is necessary to interact with the API.\nPlease refer to the README for more information."
        )]
    // MissingSessionToken(#[source] Option<anyhow::Error>),
    MissingSessionToken(#[source] Option<Box<dyn std::error::Error>>),

    #[error(
        "The year '{0}' is invalid. This should be an integer between 2015 and the current year."
    )]
    YearNotValid(u16),

    #[error(
        "AoC Error: The day number {0} is invalid. This should be an integer between 1 and 25."
    )]
    DayNotValid(u8),

    #[error("Reqwest Error: {0}")]
    FailedReqwest(reqwest::Error),

    #[error("IO Error: {0}")]
    FailedMkdir(io::Error),

    #[error("IO Error: {0}")]
    FailedTouchFile(io::Error),

    #[error("IO Error: {0}")]
    FailedWriteFile(io::Error),

    #[error("Command Execution Error: {0}\nExit Status: {1}")]
    FailedCommandExecution(String, io::Error),

    #[error("Command: {0} failed with status {1}\nError Output: {2}")]
    NonZeroExit(String, ExitStatus, String),
}

impl AdminError {
    pub fn dir_not_found<T: AsRef<str>>(var: T) -> Self {
        Self::DirectoryNotLocated(var.as_ref().to_string().into())
    }

    pub fn variable_not_set<T: AsRef<str>>(var: T) -> Self {
        Self::VariableNotSet(var.as_ref().to_string().into())
    }

    pub fn unset_session_token<T: AsRef<str>>(var: T) -> Self {
        let source_error = AdminError::variable_not_set(var.as_ref());
        Self::MissingSessionToken(Some(Box::new(source_error)))
    }

    pub fn get_chain(&self) -> Vec<&dyn Error> {
        let mut error_chain: Vec<&dyn Error> = vec![];
        let mut source: Option<&dyn Error> = Some(self);

        // Collect errors into a Vec
        while let Some(src) = source {
            error_chain.push(src);
            source = src.source();
        }

        // Return error chain
        error_chain.into_iter().collect()
    }

    pub fn print(&self) {
        let errors = self.get_chain();

        // Print errors in reverse order
        for err in errors.into_iter().rev() {
            eprintln!("{}", err);
        }
    }
}
