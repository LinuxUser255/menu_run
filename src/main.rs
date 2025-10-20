use std::io::{self, Write};
use runner::{run, Script};

mod runner;

fn prompt() {
    println!("Menu: Make a selection (A, B, C, Q/quit/exit):  ");
    let _ = std::io::stdout().flush(); // flush the stdout buffer to display the prompt immediately
}

fn main() {
    loop {
        prompt();  // display the prompt to the user
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Read error");
            continue;
        }

        match input.trim().to_lowercase().as_str() {
                "a" => {
                    println!("shell script one");
                    if let Ok(_exit_code) = run(Script::One) {
                        // Script executed successfully
                    }
                    break;
                }
                "b" => {
                    println!("shell script two");
                    if let Ok(_exit_code) = run(Script::Two) {
                        // Script executed successfully
                    }
                    break;
                }
                "c" => {
                    println!("shell script three");
                    if let Ok(_exit_code) = run(Script::Three) {
                        // Script executed successfully
                    }
                    break;
                }

                "q" | "quit" | "exit" => {
                   println!("Exiting...");
                   break;
                }
               _ => {
                  println!("Invalid selection. Please try again.");
             }
        }
     }
     println!();
}
