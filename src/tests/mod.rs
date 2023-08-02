pub mod parser_tests;
pub mod lexer_tests;
pub mod ast_tests;

#[cfg(test)]
#[ctor::ctor]
fn setup() {
   use color_eyre::owo_colors::{OwoColorize, colors::xterm::LightAquamarine};

   let _ = color_eyre::install();
   eprintln!("{}", "\nInstalling color_eyre...".bg::<LightAquamarine>().bright_purple().bold());
}
