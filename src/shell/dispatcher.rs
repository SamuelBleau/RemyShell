// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/dispatcher.rs
// Description: Command dispatcher for routing shell commands

use std::collections::HashMap;
use crate::shell::state::{ShellState};

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

type BuiltinFn = fn(&[String], &mut ShellState) -> DispatchResult;

pub struct Dispatcher {
    builtins: HashMap<String, BuiltinFn>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let mut builtins: HashMap<String, BuiltinFn> = HashMap::new();

        builtins.insert("exit".to_string(), Self::builtin_exit);
        builtins.insert("cd".to_string(), Self::builtin_cd);

        Dispatcher { builtins }
    }

    pub fn dispatch(&self, input: &str, state: &mut ShellState) -> DispatchResult {
        let tokens: Vec<String> = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if tokens.is_empty() {
            return DispatchResult::Continue;
        }


        let command = &tokens[0];
        let args = &tokens[1..];


        match self.parse_command(command) {
            DispatchCommand::Builtin(cmd) => self.execute_builtin(&cmd, &args, state),
            DispatchCommand::RemyLang(code) => self.execute_remylang(&code, state),
            DispatchCommand::External(cmd) => self.execute_external(&cmd, state),
        }
    }

    fn parse_command(&self, cmd: &str) -> DispatchCommand {

        if self.builtins.contains_key(cmd) {
            DispatchCommand::Builtin(cmd.to_string())
        } else if cmd.starts_with("remy ") {
            DispatchCommand::RemyLang(cmd.to_string())
        } else {
            DispatchCommand::External(cmd.to_string())
        }
    }

    fn execute_builtin(&self, cmd: &str, args: &[String], state: &mut ShellState) -> DispatchResult {
        if let Some(func) = self.builtins.get(cmd) {
            func(args, state)
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

    fn builtin_exit(_args: &[String], _state: &mut ShellState) -> DispatchResult {
        DispatchResult::Exit
    }

    fn builtin_cd(args: &[String], state: &mut ShellState) -> DispatchResult {
        let path = args.first().map(|s| s.as_str()).unwrap_or("");
        match state.change_directory(path) {
            Ok(_) => DispatchResult::Continue,
            Err(err) => DispatchResult::Error(err),
        }
    }
}