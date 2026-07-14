/// Déclarer les sous-modules constituant le domaine métier de RustLake.
pub mod errors;
pub mod log_event;

/// Réexporter les types principaux afin de simplifier les imports dans le reste du crate.
pub use errors::{RustLakeError, RustLakeResult};
pub use log_event::{LogEvent, Severity};