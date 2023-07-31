#![allow(unused)]
#[cfg(test)]

use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{Program, LetStatement, Identifier, Node};

#[test]
fn test_string() {
   let program: Program = Program { 
      
      statements: vec![
         Box::new(LetStatement {
            token: Token::new(TokenType::LET, "let"),

            name: Identifier {
               token: Token::new(TokenType::IDENT, "myVar"),
               value: "myVar".to_string(),
            },

            value: Some(Box::new(Identifier {
               token: Token::new(TokenType::IDENT, "anotherVar"),
               value: "anotherVar".to_string(),
            }))
         }),
      ]
   };

   if program.string() != "let myVar = anotherVar;" {
      panic!("program.string() wrong. Got: {}", program.string())
   }
}

