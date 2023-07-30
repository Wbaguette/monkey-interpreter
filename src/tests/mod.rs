pub mod parser_tests;
pub mod lexer_tests;
pub mod ast_tests;

#[test]
fn setup() {
   let _ = color_eyre::install();
}

