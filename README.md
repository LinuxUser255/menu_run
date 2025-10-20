# Menu Run

**A simple Rust application that presents an interactive menu and executes shell scripts based on user selection.**

## Execution Flow

When the program starts (`cargo run`), the following execution flow occurs:

```mermaid

```mermaid
flowchart TD
    Start([Start main.rs]) --> Loop{Enter Loop}
    Loop --> Prompt[Display Menu Prompt]
    Prompt --> Input[Read User Input]
    Input --> Parse[Convert to lowercase &amp; trim]
    Parse --> Match{Match Input}

    Match -->|a| ScriptOne[Print shell script one]
    Match -->|b| ScriptTwo[Print shell script two]
    Match -->|c| ScriptThree[Print shell script three]
    Match -->|q/quit/exit| Exit[Print Exiting...]
    Match -->|Invalid| Invalid[Print Invalid selection]

    ScriptOne --> RunOne["runner::run(Script::One)"]
    ScriptTwo --> RunTwo["runner::run(Script::Two)"]
    ScriptThree --> RunThree["runner::run(Script::Three)"]

    RunOne --> ExecOne[Execute ./scripts/shell_script_one.sh]
    RunTwo --> ExecTwo[Execute ./scripts/shell_script_two.sh]
    RunThree --> ExecThree[Execute ./scripts/shell_script_three.sh]

    ExecOne --> PrintCode1[Print exit code]
    ExecTwo --> PrintCode2[Print exit code]
    ExecThree --> PrintCode3[Print exit code]

    PrintCode1 --> Break1[Break loop]
    PrintCode2 --> Break2[Break loop]
    PrintCode3 --> Break3[Break loop]
    Exit --> Break4[Break loop]

    Invalid --> Loop

    Break1 --> End([End])
    Break2 --> End
    Break3 --> End
    Break4 --> End

```

### Detailed Flow Description

1. **Program Start**: The `main()` function in `src/main.rs` begins execution.

2. **Interactive Loop**: The program enters an infinite loop that:
   - Displays a menu prompt asking for user selection (A, B, C, or Q/quit/exit)
   - Flushes stdout to ensure the prompt is immediately visible

3. **User Input Processing**:
   - Reads a line from stdin
   - Trims whitespace and converts to lowercase for case-insensitive matching
   - If reading fails, displays an error and continues the loop

4. **Input Matching**:
   - **Option A**: Prints "shell script one", calls `runner::run(Script::One)`, breaks loop
   - **Option B**: Prints "shell script two", calls `runner::run(Script::Two)`, breaks loop
   - **Option C**: Prints "shell script three", calls `runner::run(Script::Three)`, breaks loop
   - **Quit (q/quit/exit)**: Prints "Exiting...", breaks loop
   - **Invalid input**: Prints error message, continues loop

5. **Script Execution** (via `src/runner.rs`):
   - The `run()` function receives a `Script` enum variant
   - Maps the variant to corresponding script path:
     - `Script::One` → `./scripts/shell_script_one.sh`
     - `Script::Two` → `./scripts/shell_script_two.sh`
     - `Script::Three` → `./scripts/shell_script_three.sh`
   - Spawns a bash process with the script path as argument
   - Captures and prints the exit code
   - Returns the exit code wrapped in `Result<i32>`

6. **Program Termination**: After breaking the loop, prints a blank line and exits.

## Prerequisites

- Rust toolchain (edition 2024)
- Bash shell
- Execute permissions on shell scripts: `chmod +x scripts/*.sh`

## Usage

```bash
# Build the project
cargo build

# Run the application
cargo run
```
