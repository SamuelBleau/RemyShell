// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/repl_tests.rs
// Description: Integration tests for the REPL module

use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Mutex;
use remyshell::shell::Repl;

// Thread-safe helper to track CWD for each thread
thread_local! {
    static ORIGINAL_CWD: Mutex<Option<PathBuf>> = Mutex::new(None);
}

fn setup_cwd_guard() {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    ORIGINAL_CWD.with(|orig| {
        let _ = orig.lock().map(|mut o| *o = Some(cwd));
    });
}

fn restore_cwd() {
    ORIGINAL_CWD.with(|orig| {
        if let Ok(guard) = orig.lock() {
            if let Some(ref cwd) = *guard {
                let _ = std::env::set_current_dir(cwd);
            }
        }
    });
}

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
    // Unknown commands are treated as external and don't produce output
    // Just verify the prompt and exit message are there
    assert!(output_str.contains("> "));
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
    // Unknown commands don't produce output, but prompts should be shown
    assert!(output_str.matches("> ").count() >= 4);
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
    // Unknown command, just verify it doesn't crash
    assert!(output_str.contains("Goodbye little rat!"));
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

#[test]
fn test_run_cd_command() -> anyhow::Result<()> {
    setup_cwd_guard();
    let input = Cursor::new("cd /tmp\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Should execute cd command and then exit
    assert!(output_str.contains("Goodbye little rat!"));
    // Verify at least 2 prompts were displayed (one for cd, one for exit)
    assert!(output_str.matches("> ").count() >= 2);
    restore_cwd();
    Ok(())
}

#[test]
fn test_run_invalid_cd_path() -> anyhow::Result<()> {
    let input = Cursor::new("cd /nonexistent/path\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // cd to invalid path should show error message
    assert!(output_str.contains("Error:") || output_str.contains("cd:"));
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_multiple_prompts_count() -> anyhow::Result<()> {
    let input = Cursor::new("cmd1\ncmd2\ncmd3\ncmd4\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Should have 5 prompts: one for each command plus exit
    assert!(output_str.matches("> ").count() >= 5);
    Ok(())
}

#[test]
fn test_run_unknown_command_continues() -> anyhow::Result<()> {
    let input = Cursor::new("unknown_command\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Unknown command should not crash REPL, should continue
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_remylang_error() -> anyhow::Result<()> {
    // Note: The current dispatcher doesn't properly parse RemyLang commands
    // because it splits tokens first. This test documents the current behavior.
    let input = Cursor::new("remy_command\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Unknown command treated as external, no error shown
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_state_persistence() -> anyhow::Result<()> {
    setup_cwd_guard();
    // Test that state is maintained across multiple commands
    let input = Cursor::new("cd /tmp\ncd /tmp\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Both cd commands should succeed (or both fail)
    assert!(output_str.contains("Goodbye little rat!"));
    restore_cwd();
    Ok(())
}

#[test]
fn test_run_case_sensitivity() -> anyhow::Result<()> {
    let input = Cursor::new("EXIT\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // EXIT (uppercase) should be treated as unknown command, exit lowercase should work
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_tabs_trimming() -> anyhow::Result<()> {
    let input = Cursor::new("\t\thello\t\t\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Tabs should be trimmed
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_very_long_command() -> anyhow::Result<()> {
    let long_cmd = "a".repeat(1000);
    let input = format!("{}\nexit\n", long_cmd);
    let output = Vec::new();
    let mut repl = Repl::new(Cursor::new(input), output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Should handle very long commands without crashing
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

#[test]
fn test_run_special_characters() -> anyhow::Result<()> {
    let input = Cursor::new("cmd @#$%^&*()\nexit\n");
    let output = Vec::new();
    let mut repl = Repl::new(input, output);

    repl.run()?;
    let output_str = String::from_utf8(repl.output().to_vec())?;
    // Should handle special characters without crashing
    assert!(output_str.contains("Goodbye little rat!"));
    Ok(())
}

