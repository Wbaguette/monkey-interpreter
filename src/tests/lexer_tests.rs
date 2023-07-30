#![allow(unused)]
#[cfg(test)]

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

struct Test {
   expected_tok_type: TokenType,
   expected_literal: String,
}
impl Test {
   pub fn new(exp_type: TokenType, exp_lit: &str) -> Self {
      Test { expected_tok_type: exp_type, expected_literal: exp_lit.to_string() }
   }

   pub fn new_let() -> Self {
      Test { expected_tok_type: TokenType::LET, expected_literal: "let".to_string() }
   }

   pub fn new_assign() -> Self {
      Test { expected_tok_type: TokenType::ASSIGN, expected_literal: "=".to_string() }
   }

}

#[test]
fn test_next_token_1() { 

   let input: String = String::from("=+(){},;");

   let mut lexer: Lexer = Lexer::new(input);
   let tests: Vec<Test> = vec![
      Test::new_assign(),
      Test::new(TokenType::PLUS, "+"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::EOF, ""),
   ];

   for test in tests {
      let tok: Token = lexer.next_token().unwrap();
      assert_eq!(tok.token_type, test.expected_tok_type);
      assert_eq!(tok.literal, test.expected_literal);
   }
}

#[test]
fn test_next_token_2() {
   
   let input: String = String::from(
   "  let five = 5;
      let ten = 10;
      let add = fn(x, y) {
         x + y;
      };
      let result = add(five, ten);  
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let tests: Vec<Test> = vec![
      Test::new_let(),
      Test::new(TokenType::IDENT, "five"),
      Test::new_assign(),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "ten"),
      Test::new_assign(),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "add"),
      Test::new_assign(),
      Test::new(TokenType::FUNCTION, "fn"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::PLUS, "+"),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "result"),
      Test::new_assign(),
      Test::new(TokenType::IDENT, "add"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "five"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "ten"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::SEMICOLON, ";"),
      
      Test::new(TokenType::EOF, ""),
   ];

   for test in tests {
      let tok: Token = lexer.next_token().unwrap();
      assert_eq!(tok.token_type, test.expected_tok_type);
      assert_eq!(tok.literal, test.expected_literal);
   }
}

#[test]
fn test_next_token_3() {
   
   let input: String = String::from(
   "  let five = 5;
      let ten = 10;
      let add = fn(x, y) {
         x + y;
      };
      let result = add(five, ten);  
      !-/*5;
      5 < 10 > 5;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let tests: Vec<Test> = vec![
      Test::new_let(),
      Test::new(TokenType::IDENT, "five"),
      Test::new_assign(),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "ten"),
      Test::new_assign(),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "add"),
      Test::new_assign(),
      Test::new(TokenType::FUNCTION, "fn"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::PLUS, "+"),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "result"),
      Test::new_assign(),
      Test::new(TokenType::IDENT, "add"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "five"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "ten"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::SEMICOLON, ";"),
      
      Test::new(TokenType::BANG, "!"),
      Test::new(TokenType::MINUS, "-"),
      Test::new(TokenType::SLASH, "/"),
      Test::new(TokenType::ASTERISK, "*"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::LT, "<"),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::GT, ">"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::EOF, ""),
   ];

   for test in tests {
      let tok: Token = lexer.next_token().unwrap();
      assert_eq!(tok.token_type, test.expected_tok_type);
      assert_eq!(tok.literal, test.expected_literal);
   }
}

#[test]
fn test_next_token_4() {
   
   let input: String = String::from(
   "  let five = 5;
      let ten = 10;
      let add = fn(x, y) {
         x + y;
      };
      let result = add(five, ten);  
      !-/*5;
      5 < 10 > 5;

      if (5 < 10) {
         return true;
      } else {
         return false;
      }
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let tests: Vec<Test> = vec![
      Test::new_let(),
      Test::new(TokenType::IDENT, "five"),
      Test::new_assign(),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "ten"),
      Test::new_assign(),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "add"),
      Test::new_assign(),
      Test::new(TokenType::FUNCTION, "fn"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::PLUS, "+"),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "result"),
      Test::new_assign(),
      Test::new(TokenType::IDENT, "add"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "five"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "ten"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::SEMICOLON, ";"),
      
      Test::new(TokenType::BANG, "!"),
      Test::new(TokenType::MINUS, "-"),
      Test::new(TokenType::SLASH, "/"),
      Test::new(TokenType::ASTERISK, "*"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::LT, "<"),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::GT, ">"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::IF, "if"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::LT, "<"),

      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::RETURN, "return"),
      Test::new(TokenType::TRUE, "true"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::ELSE, "else"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::RETURN, "return"),
      Test::new(TokenType::FALSE, "false"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
   
      Test::new(TokenType::EOF, ""),
   ];

   for test in tests {
      let tok: Token = lexer.next_token().unwrap();
      assert_eq!(tok.token_type, test.expected_tok_type);
      assert_eq!(tok.literal, test.expected_literal);
   }
}

#[test]
fn test_next_token_5() {
   
   let input: String = String::from(
   "  let five = 5;
      let ten = 10;
      let add = fn(x, y) {
         x + y;
      };
      let result = add(five, ten);  
      !-/*5;
      5 < 10 > 5;

      if (5 < 10) {
         return true;
      } else {
         return false;
      }

      10 == 10;
      10 != 9;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let tests: Vec<Test> = vec![
      Test::new_let(),
      Test::new(TokenType::IDENT, "five"),
      Test::new_assign(),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "ten"),
      Test::new_assign(),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "add"),
      Test::new_assign(),
      Test::new(TokenType::FUNCTION, "fn"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::IDENT, "x"),
      Test::new(TokenType::PLUS, "+"),
      Test::new(TokenType::IDENT, "y"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new_let(),
      Test::new(TokenType::IDENT, "result"),
      Test::new_assign(),
      Test::new(TokenType::IDENT, "add"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::IDENT, "five"),
      Test::new(TokenType::COMMA, ","),
      Test::new(TokenType::IDENT, "ten"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::SEMICOLON, ";"),
      
      Test::new(TokenType::BANG, "!"),
      Test::new(TokenType::MINUS, "-"),
      Test::new(TokenType::SLASH, "/"),
      Test::new(TokenType::ASTERISK, "*"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::LT, "<"),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::GT, ">"),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::IF, "if"),
      Test::new(TokenType::LPAREN, "("),
      Test::new(TokenType::INT, "5"),
      Test::new(TokenType::LT, "<"),

      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::RPAREN, ")"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::RETURN, "return"),
      Test::new(TokenType::TRUE, "true"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),
      Test::new(TokenType::ELSE, "else"),
      Test::new(TokenType::LBRACE, "{"),
      Test::new(TokenType::RETURN, "return"),
      Test::new(TokenType::FALSE, "false"),
      Test::new(TokenType::SEMICOLON, ";"),
      Test::new(TokenType::RBRACE, "}"),

      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::EQ, "=="),
      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::SEMICOLON, ";"),

      Test::new(TokenType::INT, "10"),
      Test::new(TokenType::NOTEQ, "!="),
      Test::new(TokenType::INT, "9"),
      Test::new(TokenType::SEMICOLON, ";"),


      
      Test::new(TokenType::EOF, ""),
   ];

   for test in tests {
      let tok: Token = lexer.next_token().unwrap();
      assert_eq!(tok.token_type, test.expected_tok_type);
      assert_eq!(tok.literal, test.expected_literal);
   }
}
