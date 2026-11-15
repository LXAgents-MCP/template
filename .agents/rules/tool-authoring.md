---
name: tool-authoring
description: The contract for adding a tool - one file per tool under src/controllers/, the NAME, DESCRIPTION, handler and route exports, serde + schemars parameters, and registration in TOOL_MODULES.
---

# Tool Authoring

One tool is one file. This is what keeps the tool layer reviewable, lets a scaffolding
agent delete the samples cleanly, and stops `src/routes/server.rs` growing into one `impl`
block of every tool.

## File shape

A tool lives at `src/controllers/{tool_name}.rs`, where `{tool_name}` is the registered tool
name character for character, and is declared in `src/controllers/mod.rs`. It exports:

```rust
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::MaybeSend;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::models::tool::{ToolOutcome, declaration, parameters};

pub const NAME: &str = "calculate_sum";
pub const DESCRIPTION: &str = "Add two numbers and return the sum. Requires no API key.";

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculateSumArgs {
    /// The first addend.
    pub a: f64,
    /// The second addend.
    pub b: f64,
}

pub async fn handler(Parameters(CalculateSumArgs { a, b }): Parameters<CalculateSumArgs>) -> ToolOutcome {
    Ok((a + b).to_string())
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(declaration(NAME, DESCRIPTION, parameters::<CalculateSumArgs>()), handler)
}
```

A tool with no arguments has no args struct, an `async fn handler() -> ToolOutcome`, and
`no_parameters()` in place of `parameters::<…>()`.

| Export | Required | What it is |
|---|---|---|
| `NAME` | yes | The registered tool name. Matches the filename. |
| `DESCRIPTION` | yes | One line an agent can route on without calling the tool. |
| args struct | no | `Deserialize + JsonSchema`. Omit it entirely for a tool that takes no arguments. |
| `handler` | yes | `async fn`, with `Parameters<Args>` as its only argument when there is an args struct and no argument otherwise. Returns `ToolOutcome` (`Result<String, String>`). |
| `route` | yes | `ToolRoute::new(declaration(..), handler)`, generic over the server type so the controller never imports `src/routes/`. |

The helpers `declaration`, `parameters`, `no_parameters` and the `ToolOutcome` and
`ToolModule` types live in `src/models/tool.rs`.

## The schema comes from the args struct

`parameters::<Args>()` generates the advertised JSON Schema from the struct with `schemars`.
Describe every field with a `///` doc comment: that text becomes the field's `description`,
which is what reaches the calling model as the parameter's documentation. Constraints go in
`#[schemars(...)]` attributes (`length(min = 1)`, `range(min = 0)`).

**serde does not enforce schemars constraints.** rmcp rejects a call whose arguments do not
deserialize - a missing field, a wrong type - but a `minLength` or a `range` is advertised
only. A handler checks each constraint it advertises, first, and returns an `Err` for it.

## Registration

`src/routes/server.rs` lists each tool's `route` in one array:

```rust
pub const TOOL_MODULES: &[ToolModule<TemplateServer>] = &[
    get_server_time::route,
    // ...
];
```

Adding a tool means three lines and a file: the file, its `pub mod` in
`src/controllers/mod.rs`, and its entry in `TOOL_MODULES`. Do not add a registration path
that bypasses that array; `list_tools()` and the CLI read it, and a tool registered outside it
is invisible to both.

## Authentication

A tool that needs the server's API key checks for it **inside the handler** - see
[`secrets.md`](secrets.md). Never in `Settings` or a `static`, and never as a condition on
whether the tool is registered.

## Errors

Return `Err(String)` with a message that says what was missing and what to do about it. rmcp
turns it into a tool result with `isError: true`, so there is no need to hand-build one.

Do not return an error as `Ok` text. A caller cannot tell that apart from a successful answer.
Do not `panic!` in a handler, and do not `unwrap()` anything a caller controls.

## Tests

Every tool gets coverage:

* it appears in `list_tools()` and in the client's `tools/list` (`tests/server.rs`);
* a tool with an args struct has its parameters present in the advertised input schema;
* a tool that requires the API key fails without it and succeeds with it (`tests/stdio.rs`,
  against the real binary with a per-child environment).
