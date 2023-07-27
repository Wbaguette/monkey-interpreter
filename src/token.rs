#![allow(unused)]

use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct Token {
   pub token_type: TokenType,
   pub literal: String
}

impl Token {
   pub fn new(tok_type: TokenType, lit: &str) -> Self {
      Token { token_type: tok_type, literal: lit.to_string() }
   }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
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
   FUNCTION,
   LET, 
}

impl<'a> Into<TokenType> for &'a TokenType {
   fn into(self) -> TokenType {
      match *self {
         TokenType::FUNCTION => TokenType::FUNCTION,
         TokenType::LET => TokenType::LET,
         _ => TokenType::IDENT,
      }
   }
}

lazy_static! {
   static ref KEYWORDS: HashMap<String, TokenType> = {
      let mut map = HashMap::new();
      map.insert("fn".into(), TokenType::FUNCTION);
      map.insert("let".into(), TokenType::LET);
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
