use std::process::Command;

pub enum Script {
    One,
    Two,
    Three
}

//TODO Maybe add Security: Ensure the path is absolute to prevent Directory Traversal
//     let absolute_path = path.canonicalize().map_err(|e| {
//         HardnError::ExecutionFailed(format!("Failed to resolve path {}: {}", path.display(), e))
//     })?
pub fn run(script: Script) -> std::io::Result<i32> {
    let script_name = match script {

        // need to chmod +x before running the shell scripts
        // and use the absolute path to the shell scripts as seen here.
        Script::One => "./scripts/shell_script_one.sh",
        Script::Two => "./scripts/shell_script_two.sh",
        Script::Three => "./scripts/shell_script_three.sh",
    };

    let script_path = std::path::Path::new(&script_name);

    println!("Running script: {:?}", script_path);

    // declare an output variable for the command
    let status = Command::new("bash")
        .arg(&script_path)
        .status()?;

    let exit_code = status.code().unwrap_or(-1);
    println!("Script exited with exit code: {}", exit_code);

    Ok(exit_code)
}

