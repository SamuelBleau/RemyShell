// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/repl_tests.rs
// Description: Integration tests for the REPL module

use std::io::Cursor;
use remyshell::shell::Repl;

#[test]
fn test_new_repl() {
    let input = Cursor::new("test input");
    let output = Vec::new();
    let _repl = Repl::new(input, output);
    // Verify the REPL is created successfully
}

#[test]
fn test_run_exit_command() -> anyhow::Result<()> {
    let input = Cursor::new("exit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_empty_input_then_exit() -> anyhow::Result<()> {
    let input = Cursor::new("\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("Goodbye little rat!"));
    // Verify that prompt was printed at least twice (one for empty line, one for exit)
    assert!(output_str.matches("> ").count() >= 2);
    Ok(())
}

#[test]
fn test_run_single_command_then_exit() -> anyhow::Result<()> {
    let input = Cursor::new("hello\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("hello"));
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_multiple_commands_then_exit() -> anyhow::Result<()> {
    let input = Cursor::new("command1\ncommand2\ncommand3\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("command1"));
    assert!(output_str.contains("command2"));
    assert!(output_str.contains("command3"));
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_whitespace_trimming() -> anyhow::Result<()> {
    let input = Cursor::new("  hello world  \nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("hello world"));
    Ok(())
}

#[test]
fn test_run_prompt_displayed() -> anyhow::Result<()> {
    let input = Cursor::new("exit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    assert!(output_str.contains("> "));
    Ok(())
}

