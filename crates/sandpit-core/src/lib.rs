//! Core model and backend contract for sandpit.
//!
//! This crate has no UI and no container-runtime dependency. A run is described
//! by a [`Runner`], executed by a [`Backend`], and recorded as a [`RunRecord`].

mod backend;
mod model;
mod run;

pub use backend::{Backend, BackendError};
pub use model::{Limits, Mount, Network, Runner};
pub use run::{RunId, RunRecord, RunStatus};
