// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/repl.rs
// Description: Read-Eval-Print Loop implementation for the shell

use std::io::{Write, BufRead};

pub struct Repl<R: BufRead, W: Write> {
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Repl<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Repl {
            input,
            output
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.print_prompt()?;
            let input = self.read_input()?;

            if input.is_empty() {
                continue;
            }

            if input == "exit" {
                writeln!(self.output, "Goodbye little rat!")?;
                break;
            }

            writeln!(self.output, "{}", input)?;
        }
        Ok(())
    }

    fn print_prompt(&mut self) -> anyhow::Result<()> {
        write!(self.output, "> ")?;
        self.output.flush()?;
        Ok(())
    }

    fn read_input(&mut self) -> anyhow::Result<String> {
        let mut input = String::new();
        self.input.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

// Specialized impl for testing with Vec<u8>
impl<R: BufRead> Repl<R, Vec<u8>> {
    pub fn output(&self) -> &[u8] {
        &self.output
    }
}

