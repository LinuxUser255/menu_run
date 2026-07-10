/// Prints the Command Menu banner
/// Displayed at the start of every program execution
pub fn print_banner() {
    println!(
        r#"

                    C O M M A N D    M E N U
       ══════════════════════════════════════════════════════════════
           Run Shell Scripts, Commands & other code from one place
"#
    );
}

pub fn print_modules() {
    println!("▶ Select an option:");
    println!("  a) first module");
    println!("  b) second module");
    println!("  c) third module");
    println!("  q) quit | exit");
    println!("\n═══════════════════════════════════════════");
}
