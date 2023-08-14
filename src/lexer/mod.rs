#[allow(unused)]
pub mod token;

use color_eyre::eyre::Result;
use token::{Token, TokenType, lookup};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
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

   fn peek_char(&mut self) -> char {
      if self.read_position >= self.input.len() {
         return '\0'
      } else {
         return self.input.as_bytes()[self.read_position].into();
      }
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

   fn read_string(&mut self) -> String {
      let pos: usize = self.position + 1;
      loop {
         self.read_char();
         if self.ch == '"' || self.ch == '\0' {
            break
         }
      }

      self.input[pos..self.position].into()
   }

   pub fn next_token(&mut self) -> Result<Token> {

      self.eat_whitespace();

      let tok: Token = match self.ch {
         '"' => {
            Token::new(TokenType::STRING, self.read_string().as_str())
         },
         '=' => {
            if self.peek_char() == '=' {
               let ch: char = self.ch;
               self.read_char();
               let literal: String = ch.to_string() + &self.ch.to_string();
               Token::new(TokenType::EQ, literal.as_str()
            )
            } else {
               Token::new(TokenType::ASSIGN, "=")
            }
         },
         '!' => {
            if self.peek_char() == '=' {
               let ch: char = self.ch;
               self.read_char();
               let literal: String = ch.to_string() + &self.ch.to_string();
               Token::new(TokenType::NOTEQ, literal.as_str()
            )
            } else {
               Token::new(TokenType::BANG, "!")
            }
         },
         ';' => Token::new(TokenType::SEMICOLON, ";"),
         '(' => Token::new(TokenType::LPAREN, "("),
         ')' => Token::new(TokenType::RPAREN, ")"),
         '{' => Token::new(TokenType::LBRACE, "{"),
         '}' => Token::new(TokenType::RBRACE, "}"),
         ',' => Token::new(TokenType::COMMA, ","),
         '+' => Token::new(TokenType::PLUS, "+"),
         '-' => Token::new(TokenType::MINUS, "-"),
         '/' => Token::new(TokenType::SLASH, "/"),
         '<' => Token::new(TokenType::LT, "<"),
         '>' => Token::new(TokenType::GT, ">"),
         '*' => Token::new(TokenType::ASTERISK, "*"),


         '\0' => Token::new(TokenType::EOF, ""),

         _ => {
            if self.is_letter(self.ch) {
               let identifier: &str = self.read_ident();
               let token_type: &TokenType = lookup(identifier);
               return Ok(Token::new(token_type.clone(), identifier))
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