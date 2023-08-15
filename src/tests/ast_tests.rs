#[allow(unused)]
#[cfg(test)]

use crate::lexer::token::{Token, TokenType};

#[test]
fn test_string() {
   use crate::parser::ast::{Program, LetStatement, Identifier, Node};

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
   assert_eq!(program.string(), "let myVar = anotherVar;")
}


