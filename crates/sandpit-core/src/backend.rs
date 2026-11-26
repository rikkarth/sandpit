use async_trait::async_trait;
use thiserror::Error;

use crate::{RunRecord, Runner};

/// A sandbox implementation. Podman is the first; the engine only sees this.
#[async_trait]
pub trait Backend: Send + Sync {
    /// Stable backend name, for records and diagnostics.
    fn name(&self) -> &'static str;

    /// Execute a runner and return its record.
    async fn run(&self, runner: &Runner) -> Result<RunRecord, BackendError>;
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("backend unavailable: {0}")]
    Unavailable(String),
    #[error("failed to start run: {0}")]
    Start(String),
    #[error("run failed: {0}")]
    Run(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
