use std::io;
use runner::{run, Script};

mod runner;
mod banner;

fn main() {
    loop {
        banner::display_banner();
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
                        println!("Exit code: {}", _exit_code);
                    }
                    break;
                }
                "b" => {
                    println!("shell script two");
                    if let Ok(_exit_code) = run(Script::Two) {
                        // Script executed successfully
                        println!("Exit code: {}", _exit_code);
                    }
                    break;
                }
                "c" => {
                    println!("shell script three");
                    if let Ok(_exit_code) = run(Script::Three) {
                        // Script executed successfully
                        println!("Exit code: {}", _exit_code);
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
