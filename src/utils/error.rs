use anyhow::Result;

/// Type alias for Result with anyhow::Error for consistent error handling
pub type AppResult<T> = Result<T, anyhow::Error>;