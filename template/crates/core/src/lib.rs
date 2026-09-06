//! Domain vocabulary for {{project-name}}. Types only — no I/O, no async.

mod name;

pub use self::name::{ServiceName, ServiceNameError};
