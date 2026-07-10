/// Prints the Command Menu banner
/// Displayed at the start of every program execution

pub fn display_banner() {
    println!(
        r#"
                    C O M M A N D    M E N U
          ════════════════════════════════════════════════════════
           Run Shell Scripts, Commands & other code from one place


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
