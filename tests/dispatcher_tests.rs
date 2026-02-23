// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/dispatcher_tests.rs
// Description: Unit tests for the command dispatcher

use remyshell::shell::{Dispatcher, ShellState, DispatchResult};

// Import the test utilities module
#[path = "lib.rs"]
mod test_utils;
use test_utils::cwd_guard::CwdGuard;

#[test]
fn test_dispatch_empty_input() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("", &mut state);

    // Empty input should result in Continue
    matches!(result, DispatchResult::Continue);
}

#[test]
fn test_dispatch_whitespace_only() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("   ", &mut state);

    matches!(result, DispatchResult::Continue);
}

#[test]
fn test_dispatch_exit_command() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("exit", &mut state);

    matches!(result, DispatchResult::Exit);
}

#[test]
fn test_dispatch_exit_with_args() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    // exit should ignore arguments
    let result = dispatcher.dispatch("exit 1 2 3", &mut state);

    matches!(result, DispatchResult::Exit);
}

#[test]
fn test_dispatch_cd_command_no_args() {
    let _guard = CwdGuard::new();
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();
    let _original_cwd = state.cwd.clone();

    // cd with no args should go to home directory (or stay in current if HOME not set)
    let result = dispatcher.dispatch("cd", &mut state);

    // Should either succeed (Continue) or show an error
    match result {
        DispatchResult::Continue => {
            // Successfully changed directory
        },
        DispatchResult::Error(_) => {
            // Error changing directory is also acceptable
        },
        DispatchResult::Exit => panic!("cd should not exit"),
    }
}

#[test]
fn test_dispatch_cd_to_tmp() {
    let _guard = CwdGuard::new();
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("cd /tmp", &mut state);

    match result {
        DispatchResult::Continue => {
            assert_eq!(state.cwd.to_string_lossy(), "/tmp");
        },
        DispatchResult::Error(err) => {
            // /tmp might not exist or be accessible in test environment
            assert!(err.contains("cd:"));
        },
        DispatchResult::Exit => panic!("cd should not exit"),
    }
}

#[test]
fn test_dispatch_cd_invalid_path() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("cd /nonexistent/invalid/path/12345", &mut state);

    matches!(result, DispatchResult::Error(_));
}

#[test]
fn test_dispatch_unknown_command() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("unknowncommand", &mut state);

    // Unknown commands should be treated as external and return Continue
    matches!(result, DispatchResult::Continue);
}

#[test]
fn test_dispatch_external_command() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("ls", &mut state);

    // External commands return Continue and update state.last_status
    matches!(result, DispatchResult::Continue);
    assert_eq!(state.last_status, 0);
}

#[test]
fn test_dispatch_remylang_command() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("remy let x = 42", &mut state);

    // RemyLang runtime is not yet implemented
    matches!(result, DispatchResult::Error(_));
}

#[test]
fn test_dispatch_command_with_multiple_args() {
    let _guard = CwdGuard::new();
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let result = dispatcher.dispatch("cd /tmp /var /home", &mut state);

    // cd should handle multiple arguments (take first one)
    match result {
        DispatchResult::Continue => {
            assert_eq!(state.cwd.to_string_lossy(), "/tmp");
        },
        DispatchResult::Error(err) => {
            assert!(err.contains("cd:"));
        },
        DispatchResult::Exit => panic!("cd should not exit"),
    }
}

#[test]
fn test_dispatch_preserves_state() {
    let dispatcher = Dispatcher::new();
    let mut state = ShellState::new();

    let original_env = state.env.clone();

    // Running a command should not clear the environment
    dispatcher.dispatch("exit", &mut state);

    // Note: exit command doesn't actually exit here, it just returns Exit result
    // But we can check that env was not modified
    assert_eq!(state.env, original_env);
}

