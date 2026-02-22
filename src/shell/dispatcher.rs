// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/dispatcher.rs
// Description: Command dispatcher for routing shell commands

use std::collections::HashMap;
use crate::shell::state::ShellState;

pub enum DispatchResult {
    Continue,
    Exit,
    Error(String),
}

pub enum DispatchCommand {
    Builtin(String),
    RemyLang(String),
    External(String),
}

pub struct Dispatcher {
    builtins: HashMap<String, fn(&mut ShellState) -> DispatchResult>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let mut builtins: HashMap<String, fn(&mut ShellState) -> DispatchResult> = HashMap::new();

        builtins.insert("exit".to_string(), Self::builtin_exit);

        Self { builtins }
    }

    pub fn dispatch(&self, input: &str, state: &mut ShellState) -> DispatchResult {
        let command = self.parse_command(input);
        match command {
            DispatchCommand::Builtin(cmd) => self.execute_builtin(&cmd, state),
            DispatchCommand::RemyLang(code) => self.execute_remylang(&code, state),
            DispatchCommand::External(cmd) => self.execute_external(&cmd, state),
        }
    }

    fn parse_command(&self, input: &str) -> DispatchCommand {
        let input = input.trim();

        if input.is_empty() {
            return DispatchCommand::External("".to_string());
        }

        if self.builtins.contains_key(input) {
            DispatchCommand::Builtin(input.to_string())
        } else if input.starts_with("remy ") {
            DispatchCommand::RemyLang(input[5..].to_string())
        } else {
            DispatchCommand::External(input.to_string())
        }
    }

    fn execute_builtin(&self, cmd: &str, state: &mut ShellState) -> DispatchResult {
        if let Some(func) = self.builtins.get(cmd) {
            func(state)
        } else {
            DispatchResult::Error(format!("Unknown builtin command: {}", cmd))
        }
    }

    fn execute_remylang(&self, _code: &str, _state: &mut ShellState) -> DispatchResult {
        DispatchResult::Error("RemyLang runtime not implemented yet".to_string())
    }

    fn execute_external(&self, cmd: &str, state: &mut ShellState) -> DispatchResult {
        if cmd.is_empty() {
            return DispatchResult::Continue;
        }
        // For the moment, we will just simulate external command execution
        state.last_status = 0;
        DispatchResult::Continue
    }

    fn builtin_exit(_state: &mut ShellState) -> DispatchResult {
        DispatchResult::Exit
    }
}