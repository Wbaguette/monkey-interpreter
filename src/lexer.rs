#[allow(unused)]

use color_eyre::eyre::Result;
use crate::token::{Token, TokenType, lookup};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Lexer {
   input: String,
   position: usize,
   read_position: usize,
   ch: char,
}

impl Lexer {
   pub fn new(input: String) -> Self {
      let mut l: Lexer = Lexer {
         input,
         ..Default::default()
      };
      l.read_char();
      
      l
   }

   fn read_char(&mut self) {
      if self.read_position >= self.input.len() {
         self.ch = '\0';
      } else {
         self.ch = self.input.as_bytes()[self.read_position].into();
      }
      self.position = self.read_position;
      self.read_position += 1;
   }

   fn read_ident(&mut self) -> &str {
      let pos: usize = self.position;
      while self.is_letter(self.ch) {
         self.read_char()
      }

      self.input[pos..self.position].into()
   }

   fn read_num(&mut self) -> &str {
      let pos: usize = self.position;
      while self.is_digit(self.ch) {
         self.read_char()
      }

      self.input[pos..self.position].into()
   }

   fn is_letter(&mut self, ch: char) -> bool {
      return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
   }

   fn is_digit(&mut self, ch: char) -> bool {
      return '0' <= ch && ch <= '9';
   } 

   fn eat_whitespace(&mut self) {
      loop {
         match self.ch {
            ' ' => self.read_char(),
            '\t' => self.read_char(),
            '\n' => self.read_char(),
            '\r' => self.read_char(),
            _ => break,
         }
      }
   }

   pub fn next_token(&mut self) -> Result<Token> {

      self.eat_whitespace();

      let tok: Token = match self.ch {
         '=' => Token::new(TokenType::ASSIGN, "="),
         ';' => Token::new(TokenType::SEMICOLON, ";"),
         '(' => Token::new(TokenType::LPAREN, "("),
         ')' => Token::new(TokenType::RPAREN, ")"),
         '{' => Token::new(TokenType::LBRACE, "{"),
         '}' => Token::new(TokenType::RBRACE, "}"),
         ',' => Token::new(TokenType::COMMA, ","),
         '+' => Token::new(TokenType::PLUS, "+"),
         '\0' => Token::new(TokenType::EOF, ""),

         _ => {
            if self.is_letter(self.ch) {
               let identifier: &str = self.read_ident();
               let token_type: &TokenType = lookup(identifier);
               return Ok(Token::new(token_type.into(), identifier))
            } else if self.is_digit(self.ch) {
               let literal: &str = self.read_num();
               return Ok(Token::new(TokenType::INT, literal))
            } else {
               Token::new(TokenType::ILLEGAL, &self.ch.to_string())
            }
         }

      };

      self.read_char();
      Ok(tok)
   }
}