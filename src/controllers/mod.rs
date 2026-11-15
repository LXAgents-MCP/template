//! One tool per file. Adding a tool is two edits: the new file, and its entry in
//! `TOOL_MODULES` in `src/routes/server.rs`.
//!
//! The four tools here are samples covering *takes parameters* × *requires an API key*. They are
//! deleted at scaffold time - see `PROMPT.md`.

pub mod calculate_sum;
pub mod get_secure_summary;
pub mod get_server_time;
pub mod search_secure_data;
