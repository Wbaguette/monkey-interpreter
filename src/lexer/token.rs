#![allow(unused)]

use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug)]
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

// ALL KEYWORDS SHOULD BE PATTERNED, NOT UNDER _
impl<'a> Into<TokenType> for &'a TokenType {
   fn into(self) -> TokenType {
      match *self {
         TokenType::FUNCTION => TokenType::FUNCTION,
         TokenType::LET => TokenType::LET,
         TokenType::TRUE => TokenType::TRUE,
         TokenType::FALSE => TokenType::FALSE,
         TokenType::IF => TokenType::IF,
         TokenType::ELSE => TokenType::ELSE,
         TokenType::RETURN => TokenType::RETURN,
         _ => TokenType::IDENT,
      }
   }
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
