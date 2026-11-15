//! The version, read from `Cargo.toml` at compile time.
//!
//! While this repository is a template the version is always `0.0.0`; see
//! `.agents/rules/template-mode.md`.

/// The package version, as `Cargo.toml` declares it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
