//! Allowlisted jobs the menu can run.
//! Defines the allow listed jobs that the application is allowed to run.
//! # Responsibility (SRP)
//! This module answers **what** can run: identity, labels, language kind, and
//! source paths. It should **not** spawn processes — that belongs in `runner`.
//!
//! # Security
//! Prefer a closed `enum` of jobs over accepting free-form paths from the user.
//! Menu input maps to a variant; only you choose the filesystem path.

use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// JobKind — how the runner should treat this job
// ---------------------------------------------------------------------------

/// Execution strategy. The runner will `match` on this.
///
/// TODO: When you add compiled languages, extend this enum (e.g. `Rust`, `C`)
/// and handle the new arms in `runner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobKind {
    Shell,
    Rust,
    C,
}

// ---------------------------------------------------------------------------
// Job — the allowlist
// ---------------------------------------------------------------------------

/// Closed set of runnable targets.
///
/// TODO: Add a variant per file under `modules/` that the menu should expose.
/// Suggested mapping for the current tree:
///   ShellOne   -> modules/shell/one.sh
///   ShellTwo   -> modules/shell/two.sh
///   ShellThree -> modules/shell/three.sh
///   RustHello  -> modules/rust/hello.rs   (later)
///   CHello     -> modules/c/hello.c       (later)
///
/// Extending later = new variant + fill in the `impl` arms + one menu key in `main`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    ShellOne,
    ShellTwo,
    ShellThree,
    RustHello,
    CHello,
}

impl Job {
    /// Short label for the menu / logs (e.g. "Shell: one").
    pub fn label(self) -> &'static str {
        match self {
            Job::ShellOne => "Shell: one",
            Job::ShellTwo => "Shell: two",
            Job::ShellThree => "Shell: three",
            Job::RustHello => "Rust: hello",
            Job::CHello => "C: hello",
        }
    }

    /// Which execution strategy `runner` should use.
    pub fn kind(self) -> JobKind {
        match self {
            Job::ShellOne => JobKind::Shell,
            Job::ShellTwo => JobKind::Shell,
            Job::ShellThree => JobKind::Shell,
            Job::RustHello => JobKind::Rust,
            Job::CHello => JobKind::C,
        }
    }

    /// Path to the source file, relative to the crate root
    /// (e.g. `"modules/shell/one.sh"`).
    ///
    /// Tip: return `&'static Path` via `Path::new("modules/...")` or build a
    /// `PathBuf` in `source_path` — pick one style and stay consistent.
    pub fn source_relative(self) -> &'static str {
        match self {
            Job::ShellOne => "modules/shell/shell_script_one.sh",
            Job::ShellTwo => "modules/shell/shell_script_two.sh",
            Job::ShellThree => "modules/shell/shell_script_three.sh",
            Job::RustHello => "modules/rust/hello.rs",
            Job::CHello => "modules/c/hello.c",
        }
    }

    /// Full path to the source, preferably independent of process cwd.
    ///
    /// Built as: crate root (where Cargo.toml lives) + `source_relative()`.
    /// Uses `env!("CARGO_MANIFEST_DIR")` — a compile-time path Cargo injects,
    /// not a runtime flag you pass to `cargo run`.
    pub fn source_path(self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(self.source_relative())
    }

    pub fn c_source_paths(self) -> Vec<PathBuf> {
        match self {
            Job::CHello => vec![
                self.source_path(),
                Path::new(env!("CARGO_MANIFEST_DIR")).join("modules/c/mem_demo.c"),
            ],
            _ => vec![self.source_path()],
        }
    }

    #[allow(dead_code)]
    pub fn artifact_path(self) -> Option<PathBuf> {
        match self {
            Job::ShellOne | Job::ShellTwo | Job::ShellThree => None,
            Job::RustHello => Some(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("modules/build/hello_rs")
            ),
            Job::CHello => Some(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("modules/build/hello_c"),
            ),
        }
    }
} // closes impl Job

// Optional later: private helper `fn crate_root() -> PathBuf` if you reuse
// env!("CARGO_MANIFEST_DIR") in artifact_path too.
