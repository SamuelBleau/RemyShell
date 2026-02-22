// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/repl.rs
// Description: Read-Eval-Print Loop implementation for the shell

use std::io::{self, Write};

pub struct Repl;

impl Repl {
    pub fn new() -> Self {
        Repl
    }

    pub fn run(&self) -> anyhow::Result<()> {
        loop {
            self.print_prompt();
            let input = self.read_input()?;

            if input.is_empty() {
                continue;
            }

            if input == "exit" {
                println!("Goodbye little rat!");
                break;
            }

            println!("{}", input);
        }
        Ok(())
    }

    fn print_prompt(&self) {
        print!("> ");
        io::stdout().flush().unwrap();
    }

    fn read_input(&self) -> anyhow::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}