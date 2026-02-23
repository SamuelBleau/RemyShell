// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/integration_tests.rs
// Description: Integration tests for component interaction

use std::io::Cursor;
use remyshell::shell::Repl;

// Import the test utilities module
#[path = "lib.rs"]
mod test_utils;
use test_utils::cwd_guard::CwdGuard;

#[test]
fn test_repl_cd_changes_state_across_commands() -> anyhow::Result<()> {
    let _guard = CwdGuard::new();
    // Test that state persists across multiple cd commands
    let input = Cursor::new("cd /tmp\ncd .\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // Should execute all commands without crashing
    assert!(output_str.contains("Goodbye little rat!"));
    // Should display prompts for each command
    assert!(output_str.matches("> ").count() >= 3);
    Ok(())
}

#[test]
fn test_repl_error_recovery() -> anyhow::Result<()> {
    let _guard = CwdGuard::new();
    // Test that REPL continues after encountering an error
    let input = Cursor::new("cd /nonexistent/path\ncd /tmp\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // First cd fails, second cd should still execute
    assert!(output_str.contains("Goodbye little rat!"));
    // Error message should appear
    assert!(output_str.contains("Error:") || output_str.contains("cd:"));
    Ok(())
}

#[test]
fn test_repl_command_sequence_with_mixed_types() -> anyhow::Result<()> {
    let _guard = CwdGuard::new();
    // Test a sequence of different command types
    let input = Cursor::new("unknown_cmd\nremy let x = 1\ncd /tmp\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // Should handle all command types gracefully
    assert!(output_str.contains("Goodbye little rat!"));
    assert!(output_str.matches("> ").count() >= 4);
    Ok(())
}

#[test]
fn test_repl_only_empty_lines() -> anyhow::Result<()> {
    // Test REPL with only empty lines before exit
    let input = Cursor::new("\n\n\n\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // Should handle empty lines gracefully
    assert!(output_str.contains("Goodbye little rat!"));
    // Should show 5 prompts (one for each empty line + exit)
    assert!(output_str.matches("> ").count() >= 5);
    Ok(())
}

#[test]
fn test_repl_immediate_exit() -> anyhow::Result<()> {
    // Test REPL exits immediately with exit command
    let input = Cursor::new("exit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    assert!(output_str.contains("Goodbye little rat!"));
    // Should have exactly 1 prompt
    assert_eq!(output_str.matches("> ").count(), 1);
    Ok(())
}

#[test]
fn test_repl_cd_to_various_locations() -> anyhow::Result<()> {
    let _guard = CwdGuard::new();
    // Test cd to multiple locations
    let input = Cursor::new("cd /tmp\ncd .\ncd /\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // Should complete all commands
    assert!(output_str.contains("Goodbye little rat!"));
    // Should have prompts for all commands
    assert!(output_str.matches("> ").count() >= 4);
    Ok(())
}

#[test]
fn test_repl_output_format_consistency() -> anyhow::Result<()> {
    // Test that prompts are consistently formatted
    let input = Cursor::new("cmd1\ncmd2\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;

    // Check that prompts follow expected format (path> )
    let prompt_count = output_str.matches("> ").count();
    assert!(prompt_count >= 3);

    // Check that goodbye message is present
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

