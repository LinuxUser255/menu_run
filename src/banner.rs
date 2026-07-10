/// Prints the LinuxSecure banner
/// Displayed at the start of every program execution
pub fn print_banner() {
    println!(
        r#"

                    C O M M A N D    M E N U
    ════════════════════════════════════════════════════
           Run Shell Scripts, Commands & other code from one place

"#
    );
}

pub fn display_banner() {
    println!(
        r#"
═══════════════════════════════════════════════════════════════════════════════

▶ Select an option:

  a) first module
  b) second module
  c) third module
  q) quit | exit

═══════════════════════════════════════════════════════════════════════════════

"#
    );
}
