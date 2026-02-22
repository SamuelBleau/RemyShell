// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/mod.rs
// Description: Shell module providing core shell functionality

mod repl;
mod state;
mod command;
mod dispatcher;

pub use repl::Repl;
pub use state::ShellState;
pub use command::{Command, parse_command};
