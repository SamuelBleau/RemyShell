// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/state_tests
// Description: Integration tests for shell state

use remyshell::shell::ShellState;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn new_state_defaults() {
    let state = ShellState::new();
    let expected_cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    assert_eq!(state.cwd, expected_cwd);
    assert_eq!(state.last_status, 0);
}

#[test]
fn new_state_env_snapshot() {
    let expected_env: HashMap<String, String> = std::env::vars().collect();
    let state = ShellState::new();
    assert_eq!(state.env, expected_env);
}

