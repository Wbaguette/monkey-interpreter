#![allow(unused)]

use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Token {
   pub token_type: TokenType,
   pub literal: String
}
impl Token {
   pub fn new(tok_type: TokenType, lit: &str) -> Self {
      Token { token_type: tok_type, literal: lit.to_string() }
   }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum TokenType {
   // UNKNOWN: Only a thing when initializing the parser
   UNKNOWN,

   // Operators
   ILLEGAL,
   EOF,
   IDENT,
   INT,
   ASSIGN,
   PLUS,
   COMMA,
   SEMICOLON, 
   LPAREN,
   RPAREN,
   LBRACE,
   RBRACE,
   MINUS,
   BANG, 
   ASTERISK,
   SLASH,
   LT, 
   GT, 
   EQ,
   NOTEQ,

   // Keywords
   FUNCTION,
   LET,
   TRUE,
   FALSE,
   IF,
   ELSE,
   RETURN,
}

lazy_static! {
   static ref KEYWORDS: HashMap<String, TokenType> = {
      let mut map = HashMap::new();
      map.insert("fn".into(), TokenType::FUNCTION);
      map.insert("let".into(), TokenType::LET);
      map.insert("true".into(), TokenType::TRUE);
      map.insert("false".into(), TokenType::FALSE);
      map.insert("if".into(), TokenType::IF);
      map.insert("else".into(), TokenType::ELSE);
      map.insert("return".into(), TokenType::RETURN);

      map
   };
}

pub fn lookup(ident: &str) -> &TokenType {
   if let Some(identifier) = KEYWORDS.get(ident) {
      return identifier;
   } else {
      return &TokenType::IDENT
   }
}
