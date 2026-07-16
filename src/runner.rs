//#![process execution for allowlisted jobs.
//!
//! # Responsibility (SRP)
//! Owns process execution for allow listed jobs.
//! This module answers **how** to run a job: spawn processes, compile sources,
//! and return exit codes.
//! It should **not** own menu I/O or decide which menu key means which job.
//!
//! # Security Implementation
//! See docs/SECURITY_TODO.md
//! - Accept only a `Job` from the allowlist; never accept raw user-provided paths.
//! - `Job` is a closed enum: menu input selects a known variant, not an arbitrary path.
//! - Pass executable names and paths as separate `Command` arguments.
//! - Avoid `sh -c`, string-built commands, or forwarding untrusted text to a shell.
//! - This is not a sandbox: allowlisted scripts/programs still run with this
//!   process user's permissions.
//! - Paths currently come from `Job` literals. Optional hardening: canonicalize
//!   source paths and ensure they stay under `modules/`; canonicalize artifacts
//!   and ensure they stay under `modules/build/`.

use std::fs;
use std::io;
use std::process::Command;

use crate::jobs::{Job, JobKind};

pub fn run(job: Job) -> io::Result<i32> {
    match job.kind() {
        JobKind::Shell => run_shell(job),
        JobKind::Rust => run_rust(job),
        JobKind::C => run_c(job),
    }
}

/// Run a shell module with bash
fn run_shell(job: Job) -> io::Result<i32> {
    let path = job.source_path();

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("script not found: {}", path.display()),
        ));
    }

    // Fixed program + path as one arg (no user text in a shell string).
    let status = Command::new("bash").arg(&path).status()?;
    Ok(status.code().unwrap_or(1))
}

/// Compile a Rust module with `rustc`, then run the binary.
/// source exists? → ensure build dir → rustc → if fail, Ok(code) → run binary
fn run_rust(job: Job) -> io::Result<i32> {
    // Source path (same idea as run_shell; define BEFORE you use it) ---
    let source = job.source_path();

    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("source not found: {}", source.display()),
        ));
    }

    // Artifact, (compiled code), path (Option → PathBuf, or Err if None) ---
    let artifact = job.artifact_path().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Rust job has no artifact path",
        )
    })?;

    // Ensure modules/build (parent of the binary) exists ---
    if let Some(parent) = artifact.parent() {
        fs::create_dir_all(parent)?;
    }
    // Compile: rustc <source> -o <artifact> ---
    let compile = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&artifact)
        .status()?;

    // If compile failed, return that exit code and STOP ---
    if !compile.success() {
        return Ok(compile.code().unwrap_or(1));
    }

    // Prefer Ok(non_zero) over Err — the process *ran*; it just failed.
    let status = Command::new(&artifact).status()?;
    Ok(status.code().unwrap_or(1))

}

fn run_c(job: Job) -> std::io::Result<i32> {
    // compile all C source files for this job, then run the ouput binary
    let sources = job.c_source_paths();
    let artifact = job.artifact_path().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "C job has no artifact path",
        )
    })?;

    for source in &sources {
        if !source.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("source not found: {}", source.display()),
            ));
        }

    }

    if let Some(parent) = artifact.parent() {
        fs::create_dir_all(parent)?;
    }

    if sources.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "C job has no source files",
        ));
    }

    let compile = Command::new("gcc") // or "gcc"
        .args(&sources)
        .arg("-o")
        .arg(&artifact)
        .status()?;

    if !compile.success() {
        return Ok(compile.code().unwrap_or(1));
    }

    let status = Command::new(&artifact).status()?;
    Ok(status.code().unwrap_or(1))
}
