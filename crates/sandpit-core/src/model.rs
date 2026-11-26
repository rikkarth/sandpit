use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// A reusable job definition: what to run, where, and with which limits.
///
/// Deserialized from TOML with unknown fields rejected, so typos fail loudly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Runner {
    /// Optional human label; the file name is the runner's identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Image reference. The engine requires a pinned digest.
    pub image: String,
    /// Command executed inside the container.
    pub command: Vec<String>,
    /// Prompt handed to the agent, if the command expects one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Host paths exposed inside the container.
    #[serde(default)]
    pub mounts: Vec<Mount>,
    /// Network access for the run. Defaults to none.
    #[serde(default)]
    pub network: Network,
    #[serde(default)]
    pub limits: Limits,
    /// Container path bind-mounted to the run's artifacts directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<PathBuf>,
}

/// A host path made available inside the container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mount {
    pub host: PathBuf,
    pub container: PathBuf,
    #[serde(default)]
    pub read_only: bool,
}

/// Network access granted to a run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    /// No network at all (`--network=none`).
    #[default]
    None,
    /// Outbound access through the backend's default network.
    Egress,
    /// A named network managed outside sandpit.
    Named(String),
}

/// Resource limits for a run.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    /// Wall-clock limit, written as a human duration such as `"30m"`.
    #[serde(
        default,
        with = "humantime_serde::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<Duration>,
    /// CPU cores available to the run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpus: Option<f32>,
}
