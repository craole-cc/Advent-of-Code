pub mod external {
  #[cfg(feature = "time")]
  pub use chrono::{
    DateTime,
    Duration,
    Local,
    Utc,
  };
  #[cfg(all(feature = "tracing", feature = "time"))]
  pub use tracing_subscriber::fmt::time::{
    ChronoLocal as TracingLocal,
    ChronoUtc as TracingUtc,
    SystemTime as TracingSysTime,
  };
  pub use {
    super::super::utils::*,
    std::time::{
      Duration as StdDuration,
      SystemTime as StdSysTime,
    },
  };
}

pub mod internal {
  pub use crate::prelude::*; // TODO: Check this
  pub use {
    chrono::prelude::*,
    std::time::{
      Duration as StdDuration,
      SystemTime as StdSysTime,
    },
  };
}
