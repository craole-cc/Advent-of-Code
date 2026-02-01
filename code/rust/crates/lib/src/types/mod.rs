pub mod aoc;
pub mod package;
mod environment;

/// Struct for formatting an incremental package name with leading zeros.
#[derive(Debug, PartialEq)]
pub struct Package {
    /// The base name for the formatter.
    pub base_name: String,
    /// The incremental number for the formatter.
    pub number: Option<u8>,
    /// The padding width for the incremental number.
    pub digits: Option<u8>,
    /// Of type `Aoc`.
    pub is_aoc: bool,
    /// Advent of Code Specification.
    pub aoc: AoC,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AoC {
    pub token: String,
    pub day: u8,
    pub year: u16,
}

pub struct GlobalEnvironment {
    pub cargo_meta: cargo_metadata::Metadata,
    pub workspace: std::path::PathBuf,
    pub aoc_home: std::path::PathBuf,
    pub aoc_session_token: String,
    pub establish_environment: (),
}
