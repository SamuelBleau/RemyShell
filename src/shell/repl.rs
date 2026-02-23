// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/repl.rs
// Description: Read-Eval-Print Loop implementation for the shell

use std::io::{Write, BufRead};
use crate::shell::ShellState;
use crate::shell::dispatcher::DispatchResult;
use crate::shell::dispatcher::Dispatcher;

pub struct Repl<R: BufRead, W: Write> {
    input: R,
    output: W,
    state: ShellState,
    dispatcher: Dispatcher
}

impl<R: BufRead, W: Write> Repl<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Repl {
            input,
            output,
            state: ShellState::new(),
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.print_prompt()?;
            let input = self.read_input()?;
            let result = self.dispatcher.dispatch(&input, &mut self.state);

            match result {
                DispatchResult::Continue => {}
                DispatchResult::Exit => {
                    writeln!(self.output, "Goodbye little rat!")?;
                    break
                },
                DispatchResult::Error(err) => {
                    writeln!(self.output, "Error: {}", err)?;
                }
            }
        }
        Ok(())
    }

    fn print_prompt(&mut self) -> anyhow::Result<()> {
        write!(self.output, "{}> ", self.state.cwd.display())?;
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

