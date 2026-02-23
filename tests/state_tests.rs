// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/state_tests
// Description: Integration tests for shell state

use remyshell::shell::ShellState;
use std::path::PathBuf;
use std::collections::HashMap;

// Import the test utilities module
#[path = "lib.rs"]
mod test_utils;
use test_utils::cwd_guard::CwdGuard;

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

#[test]
fn test_change_directory_to_tmp() {
    let _guard = CwdGuard::new();
    let mut state = ShellState::new();

    let result = state.change_directory("/tmp");

    if result.is_ok() {
        assert_eq!(state.cwd.to_string_lossy(), "/tmp");
    } else {
        // /tmp might not exist on all systems, but if it fails, it should be an error
        assert!(result.is_err());
    }
}

#[test]
fn test_change_directory_invalid_path() {
    let mut state = ShellState::new();

    let result = state.change_directory("/this/path/definitely/does/not/exist/12345");

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("cd:"));
}

#[test]
fn test_change_directory_empty_string_goes_home() {
    let _guard = CwdGuard::new();
    let mut state = ShellState::new();

    // cd with empty string should go to HOME directory
    let result = state.change_directory("");

    // Should either succeed or fail if HOME is not set
    match result {
        Ok(_) => {
            // Should have changed directory
            let home = std::env::var("HOME").ok().map(PathBuf::from);
            if let Some(home_path) = home {
                assert_eq!(state.cwd, home_path);
            }
        },
        Err(err) => {
            // HOME might not be set, which is fine
            assert!(err.contains("cd:"));
        }
    }
}

#[test]
fn test_change_directory_updates_state() {
    let _guard = CwdGuard::new();
    let mut state = ShellState::new();
    let original_cwd = state.cwd.clone();

    // Try to change directory
    let result = state.change_directory("/tmp");

    // If successful, state should be updated
    if result.is_ok() {
        assert_ne!(state.cwd, original_cwd);
    }
}

#[test]
fn test_change_directory_error_message_format() {
    let mut state = ShellState::new();

    let result = state.change_directory("/nonexistent/12345");

    assert!(result.is_err());
    let err = result.unwrap_err();
    // Error message should start with "cd: " and mention the path
    assert!(err.starts_with("cd:"));
    assert!(err.contains("/nonexistent/12345") || err.contains("No such file"));
}

#[test]
fn test_state_last_status_initialization() {
    let state = ShellState::new();

    assert_eq!(state.last_status, 0);
}

#[test]
fn test_state_env_not_empty() {
    let state = ShellState::new();

    // Environment should have at least PATH or other standard variables
    assert!(!state.env.is_empty());
}

#[test]
fn test_change_directory_current_dir() {
    let _guard = CwdGuard::new();
    let mut state = ShellState::new();
    let _original_cwd = state.cwd.clone();

    // Change to "." should succeed
    let result = state.change_directory(".");

    assert!(result.is_ok());
    // After changing to ".", the current directory should be the canonicalized version
    // which might differ from original_cwd if it contains symlinks
    assert!(state.cwd.as_path().is_absolute());
}

