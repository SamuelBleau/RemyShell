// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: shell/state.rs
// Description: Shell state management and persistence

use std::collections::HashMap;
use std::path::PathBuf;
pub struct ShellState {
    pub cwd: PathBuf,
    pub last_status: i32,
    pub env: HashMap<String, String>,
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            last_status: 0,
            env: std::env::vars().collect(),
        }
    }

    pub fn change_directory(&mut self, path: &str) -> Result<(), String> {
        let target = if path.is_empty() {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"))
        } else {
            PathBuf::from(path)
        };

        std::env::set_current_dir(&target)
            .map_err(|e| format!("cd: {}: {}", target.display(), e))?;

        self.cwd = target;
        Ok(())
    }
}