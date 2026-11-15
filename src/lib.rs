//! `template`: an MCP server and a CLI over one implementation.
//!
//! The two binaries are thin: `src/main.rs` (`template-server`) and `src/cli.rs` (`template`)
//! both call into this library. Imports flow downward only:
//! `routes` → `controllers` → `models` → `config`.

#![forbid(unsafe_code)]

pub mod config;
pub mod controllers;
pub mod models;
pub mod routes;
