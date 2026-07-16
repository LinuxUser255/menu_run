//! Menu loop: read a key, map it to a `Job`, ask `runner` to execute it.
//!
//! # Responsibility (SRP)
//! - Own stdin/stdout interaction and the key → job mapping.
//! - Do **not** build filesystem paths or spawn processes here (use `jobs` + `runner`

mod runner;
mod banner;
mod jobs;

use std::io::{self, Write};
use jobs::Job;
use runner::run;
use crate::banner::{print_banner, print_modules};

fn prompt() {
    println!("Menu: Make a selection (A, B, C, Q/quit/exit):  ");
    let _ = std::io::stdout().flush(); // flush the stdout buffer to display the prompt immediately
    print_banner();
    print_modules();
}

fn main() -> io::Result<()>{
    loop {
        prompt();  // display the prompt to the user

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Read error");
            continue;
        }

        match input.trim().to_lowercase().as_str() {
            "a" => {
                // Template for other keys: run(Job::…) then handle Result.
                match run(Job::ShellOne) {
                    Ok(code) => println!("{} exited with {code}", Job::ShellOne.label()),
                    Err(e) => eprintln!("failed: {e}"),
                }
            }

            "b" => {
                match run(Job::ShellTwo) {
                    Ok(code) => println!("{} exited with {code}", Job::ShellTwo.label()),
                    Err(e) => eprintln!("failed: {e}"),
                }
            }
            "c" => {
                match run(Job::ShellThree) {
                    Ok(code) => println!("{} exited with {code}", Job::ShellThree.label()),
                    Err(e) => eprintln!("failed: {e}"),
                }
            }
            "d" => { match run(Job::RustHello) {
                Ok(code) => println!("{} exited with {code}", Job::RustHello.label()),
                Err(e) => eprintln!("failed: {e}"),
            } }
            "e" => { match run(Job::CHello) {
                Ok(code) => println!("{} exited with {code}", Job::CHello.label()),
                Err(e) => eprintln!("failed: {e}"),
            } }

            "q" | "quit" | "exit" => {
                println!("Exiting..");
                break;
            }

            _ => {
                println!("Invalid selection.");
            }
        }

        println!();
    }

    Ok(())
}


