#![allow(unused)]
#[cfg(test)]

use crate::parser::ast::Program;
use crate::parser::ast::{Statement, LetStatement, Node};
use crate::lexer::Lexer;
use crate::parser::Parser;

struct IDtest {
   expected_identifier: String,
}

impl IDtest {
   pub fn new(expected_identifier: &str) -> Self {
      IDtest { expected_identifier: expected_identifier.into() }
   }
}

fn test_let_statement(statement: &Box<dyn Statement>, name: &str) {
   if statement.token_literal() != "let" {
      panic!("statement.token_literal() is {}. Expected 'let'", statement.token_literal());
   }

   if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
      
      if let_stmt.name.value != name.to_string() {
         panic!("LetStatement.name.value is {}. Expected: {}", let_stmt.name.value, name)
      }
      if let_stmt.name.token_literal() != name {
         panic!("LetStatement.name.token_literal() is {}. Expected: {}", let_stmt.name.token_literal(), name)
      }

   } else {
      panic!("statement is a not a LetStatement.")
   }
   
}

fn check_parser_errors(parser: &Parser) {
   let errors: &Vec<String> = parser.errors();

   if errors.len() == 0 {
      return;
   }

   let mut error_msg: String = String::from(format!("Parser has {} error(s). ", errors.len()));
   for error in errors {
      error_msg.push_str(format!("\nParser has error: {}", error).as_str())
   }
   panic!("{}", error_msg);
}

#[test]
fn test_let_statements() {
   
   let input: String = String::from(
   "  let x = 5;
      let y = 10;
      let foobar = 838383;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);

   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 

   check_parser_errors(&parser);

   if program.statements.len() != 3 {
      panic!("program.statements contains {} statements. Expected 3 statements", program.statements.len())
   }

   let tests: Vec<IDtest> = vec![
      IDtest::new("x"),
      IDtest::new("y"),
      IDtest::new("foobar"),
   ];

   for (idx, test) in tests.iter().enumerate() {
      let statement: &Box<dyn Statement> = program.statements.get(idx).unwrap();
      test_let_statement(statement, &test.expected_identifier)
   }

}

#[test]
fn test_return_statements() {
   
   let input: String = String::from(
   "  return 5;
      return 10;
      return 993322;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);

   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 

   check_parser_errors(&parser);

   if program.statements.len() != 3 {
      panic!("program.statements contains {} statements. Expected 3 statements", program.statements.len())
   }

   
   

}