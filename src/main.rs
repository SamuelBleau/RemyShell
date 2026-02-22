// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: main.rs
// Description: Entry point for the RemyShell application

mod shell;

fn main() {
    println!("Welcome to RemyShell!");
    let shell = shell::Repl::new();
    let _ = shell.run();
}