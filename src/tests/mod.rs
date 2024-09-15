pub mod parser_tests;
pub mod lexer_tests;
pub mod ast_tests;
pub mod evaluator_tests;
pub mod object_test;

#[cfg(test)]
#[ctor::ctor]
fn setup() {
   use color_eyre::owo_colors::OwoColorize;

   let _ = color_eyre::install();
   eprintln!("{}", "\nInstalling color_eyre...".bright_purple().bold());
}
