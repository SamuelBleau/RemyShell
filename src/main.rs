// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: main.rs
// Description: Entry point for the RemyShell application

use remyshell::shell;
use std::io::{self, BufReader};

fn main() -> anyhow::Result<()> {
    println!("Welcome to RemyShell!");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();

    let mut repl = shell::Repl::new(reader, stdout);
    repl.run()?;

    Ok(())
}