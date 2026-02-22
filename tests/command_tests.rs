// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/command_tests
// Description: Integration tests for command parsing

use remyshell::shell::Command;
use remyshell::shell::parse_command;

#[test]
fn parse_exit_command() {
    match parse_command("exit") {
        Command::Exit => {}
        _ => panic!("expected exit command"),
    }
}

#[test]
fn parse_empty_command() {
    match parse_command("") {
        Command::Empty => {}
        _ => panic!("expected empty command"),
    }
}

#[test]
fn parse_raw_command() {
    match parse_command("hello world") {
        Command::Raw(value) => assert_eq!(value, "hello world"),
        _ => panic!("expected raw command"),
    }
}
