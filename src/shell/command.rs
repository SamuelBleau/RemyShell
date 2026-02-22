// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/command.rs
// Description: Command parsing and execution logic

pub enum Command {
    Exit,
    Empty,
    Raw(String),
}

pub fn parse_command(input: &str) -> Command {
    match input {
        "exit" => Command::Exit,
        "" => Command::Empty,
        other => Command::Raw(other.to_string()),
    }
}