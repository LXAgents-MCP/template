---
name: secrets
description: One unified API key for the whole server - read it from the environment inside the handler, fail descriptively, and never log or echo it.
---

# Secrets

This server authenticates with **one unified key for the whole server**, not one key
per tool. It is read from the `API_KEY` environment variable.

## Read it inside the handler, at call time

```rust
pub async fn handler(Parameters(SearchSecureDataArgs { query }): Parameters<SearchSecureDataArgs>) -> ToolOutcome {
    if std::env::var("API_KEY").map_or(true, |key| key.is_empty()) {
        return Err(
            "search_secure_data requires an API key. Set the API_KEY environment variable before starting the server."
                .to_owned(),
        );
    }
    // ...
}
```

Never in `Settings`, a `static`, a `OnceLock` or a field of the server. A key captured at
startup is the value that happened to be set when the process began, and it would sit in
memory beside everything else `Settings` holds - including in a `Debug` print of it.

## Registration never depends on the key

Every tool is registered whether or not a key is present. A server that hides its
authenticated tools when the key is missing reports "no such tool" to the caller -
which is indistinguishable from the tool not existing, and sends whoever is debugging
it looking in the wrong place entirely.

The tool is always listed; calling it without a key is what fails, and it fails saying
so.

## The failure must be descriptive

A missing key returns an `Err` naming the tool, the variable, and the fix, which the caller
receives as an `isError` result. Not
`"unauthorized"`, not `"missing config"`, and never a silent empty result.

## Never disclose the key

* Do not log it, at any level, on any transport.
* Do not include it in tool output, an error message, or a health check response.
* Do not write it into a test fixture, a wiki page, or a commit.
* `.env` is gitignored and stays that way.

A test that needs a key starts the server binary as a child process with `API_KEY` set to an
obvious dummy (`tests/stdio.rs`). No test sets or removes a variable in its own process.
