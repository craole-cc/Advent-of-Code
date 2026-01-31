use std::{env::args, error::Error, str::FromStr};

pub fn parse_arguments() -> Result<u8, Box<dyn Error>> {
    args()
        .nth(1)
        .and_then(|arg| u8::from_str(&arg).ok())
        .ok_or("Error: Please provide a valid day number.".into())
}
