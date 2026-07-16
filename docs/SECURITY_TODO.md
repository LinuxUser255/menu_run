# Security Hardening Checklist

This project intentionally runs allowlisted local modules, including shell scripts and compiled programs. Treat every file under `modules/` as trusted code.

## Threat Model

- [ ] Document that this tool is for trusted local modules only.
- [ ] Document that this is not a sandbox.
- [ ] Document that anyone who can modify allowlisted module files can execute code as the current user.
- [ ] Decide whether modules are allowed to access the network.
- [ ] Decide whether modules are allowed to read/write outside the project directory.
- [ ] Decide whether this tool should ever run in shared or multi-user environments.

## File and Directory Permissions

- [ ] Ensure untrusted users cannot write to `modules/`.
- [ ] Ensure untrusted users cannot write to `modules/shell/`.
- [ ] Ensure untrusted users cannot write to `modules/rust/`.
- [ ] Ensure untrusted users cannot write to `modules/c/`.
- [ ] Ensure untrusted users cannot write to `modules/build/`.
- [ ] Ensure untrusted users cannot write to `src/`.
- [ ] Ensure untrusted users cannot write to `Cargo.toml` or `Cargo.lock`.
- [ ] Avoid running this program as `root` or administrator.
- [ ] Consider checking ownership and permissions before running modules.

## Command Execution

- [ ] Continue using `Command::new(...).arg(...)` instead of building command strings.
- [ ] Do not use `sh -c` with formatted strings.
- [ ] Do not pass raw user input into shell commands.
- [ ] Keep menu input mapped only to fixed `Job` enum variants.
- [ ] Keep module paths defined in code, not supplied by users.
- [ ] Consider using absolute paths for trusted executables such as `bash`, `rustc`, and `gcc`.
- [ ] Consider using a controlled `PATH`, for example `/usr/bin:/bin`.
- [ ] Consider clearing or minimizing inherited environment variables for child processes.

## Path Safety

- [ ] Canonicalize source paths before execution or compilation.
- [ ] Verify source paths stay under the expected `modules/` directory.
- [ ] Verify shell scripts stay under `modules/shell/`.
- [ ] Verify Rust sources stay under `modules/rust/`.
- [ ] Verify C sources stay under `modules/c/`.
- [ ] Validate artifact parent directories before compiling.
- [ ] Ensure compiled artifacts are written only under `modules/build/` or another controlled build directory.
- [ ] Reject paths that escape allowed directories.
- [ ] Be careful with symlinks in source and build directories.
- [ ] Consider failing if allowlisted source files or build paths are symlinks.

## Build Artifact Safety

- [ ] Use a private build directory controlled by the application.
- [ ] Consider moving build output to `target/menu_os_build/` instead of `modules/build/`.
- [ ] Remove stale artifacts before compiling.
- [ ] Ensure artifacts are not writable by untrusted users.
- [ ] Avoid executing artifacts from world-writable directories.
- [ ] Consider unique artifact names per job.
- [ ] Consider unique temporary build directories per run.
- [ ] Protect against artifact replacement between compile and execution.

## Environment Hardening

- [ ] Review which environment variables child processes inherit.
- [ ] Avoid passing secrets such as API keys, tokens, or credentials to modules.
- [ ] Consider using `env_clear()` for child processes.
- [ ] If using `env_clear()`, explicitly restore only required variables.
- [ ] Set a minimal safe `PATH`.
- [ ] Consider removing variables that affect dynamic loading or compiler behavior.
- [ ] Review variables such as `LD_PRELOAD`, `DYLD_*`, `RUSTFLAGS`, `CC`, and `CFLAGS`.

## Compiler Safety

- [ ] Treat Rust and C source files as trusted executable code.
- [ ] Do not compile untrusted C or Rust modules without sandboxing.
- [ ] Consider pinning compiler paths or validating compiler locations.
- [ ] Consider logging compiler path/version before use.
- [ ] Consider limiting compiler environment variables.
- [ ] Consider whether compiler output should be captured and logged.
- [ ] Consider whether compilation should have a timeout.

## Runtime Limits

- [ ] Add execution timeouts for shell scripts.
- [ ] Add execution timeouts for compiled Rust programs.
- [ ] Add execution timeouts for compiled C programs.
- [ ] Consider limiting stdout/stderr output size.
- [ ] Consider limiting CPU and memory usage if running on Unix-like systems.
- [ ] Consider running modules in a subprocess group so child processes can be terminated.
- [ ] Decide how to handle programs that never exit.

## Sandboxing

- [ ] Decide whether true sandboxing is required.
- [ ] If untrusted modules are ever supported, use OS-level sandboxing.
- [ ] Consider containers for untrusted modules.
- [ ] Consider a dedicated low-privilege user account for module execution.
- [ ] Consider filesystem isolation.
- [ ] Consider network isolation.
- [ ] Consider seccomp, namespaces, chroot, pledge/unveil, or platform-specific sandboxing where appropriate.
- [ ] Do not rely on Rust code structure alone as a security boundary.

## Logging and Auditability

- [ ] Log which job was selected.
- [ ] Log the resolved source path.
- [ ] Log the resolved artifact path for compiled jobs.
- [ ] Log compiler exit codes.
- [ ] Log module exit codes.
- [ ] Avoid logging secrets from environment variables.
- [ ] Consider adding a verbose/debug mode.
- [ ] Consider keeping logs outside writable module directories.

## Error Handling

- [ ] Keep returning `Err` for infrastructure failures such as missing files or failed process start.
- [ ] Keep returning `Ok(exit_code)` when the child process ran and exited non-zero.
- [ ] Make error messages clear but avoid leaking unnecessary sensitive paths in production.
- [ ] Distinguish compile failure from runtime failure in user-facing output.
- [ ] Consider custom error types if error handling grows.

## Dependency and Supply Chain

- [ ] Keep Rust toolchain updated.
- [ ] Review dependencies before adding them.
- [ ] Run `cargo audit` if third-party dependencies are introduced.
- [ ] Commit `Cargo.lock` for reproducible application builds.
- [ ] Consider CI checks for formatting, linting, and tests.
- [ ] Consider verifying module source files in code review.

## Testing

- [ ] Add tests for menu input mapping.
- [ ] Add tests that unknown input does not execute anything.
- [ ] Add tests for path validation helpers.
- [ ] Add tests that source paths cannot escape allowed directories.
- [ ] Add tests for missing source files.
- [ ] Add tests for missing artifact paths.
- [ ] Add tests for compiler failure handling.
- [ ] Add tests for non-zero runtime exit codes.
- [ ] Add tests for environment sanitization if implemented.

## Documentation

- [ ] Document the security model in `README.md`.
- [ ] Document how to add a new trusted module safely.
- [ ] Document that module files must be reviewed before use.
- [ ] Document required filesystem permissions.
- [ ] Document whether this tool is intended for local use only.
- [ ] Document known limitations and non-goals.

## Release Readiness

- [ ] Confirm no untrusted path input reaches `Command`.
- [ ] Confirm no formatted shell command strings are used.
- [ ] Confirm all module paths are allowlisted.
- [ ] Confirm build artifacts are written to controlled directories.
- [ ] Confirm this is not run with elevated privileges.
- [ ] Confirm security assumptions are documented.
- [ ] Confirm dangerous future features are gated behind explicit review.