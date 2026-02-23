// RemyShell - A developer-oriented terminal written in Rust
// Copyright (c) 2026 Samuel Bleau
// Licensed under the MIT License - see LICENSE file for details
//
// File: tests/cwd_guard.rs
// Description: Global guard for managing current working directory in tests

use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global mutex to serialize access to the current working directory
/// This prevents race conditions when multiple tests run in parallel
/// and modify the process's current directory
static CWD_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// RAII guard that saves the current directory on creation and restores it on drop
pub struct CwdGuard {
    original_cwd: PathBuf,
    _lock: std::sync::MutexGuard<'static, ()>,
}

impl CwdGuard {
    /// Create a new CWD guard that saves the current directory
    /// This acquires a global lock to ensure only one test modifies the CWD at a time
    pub fn new() -> Self {
        // Acquire the lock - this blocks until other tests are done with CWD
        let lock = CWD_LOCK.lock().unwrap();

        // Save the current working directory
        let original_cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        CwdGuard {
            original_cwd,
            _lock: lock,
        }
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        // Restore the original directory when the guard is dropped
        let _ = std::env::set_current_dir(&self.original_cwd);
    }
}

