#![allow(unused)]

pub mod ast;

use std::vec;

use crate::parser::ast::{Program, Identifier, LetStatement, Statement, ReturnStatement};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use color_eyre::Result;

#[derive(Debug)]
pub struct Parser {
   lexer: Lexer,
   cur_token: Token,
   peek_token: Token,
   pub errors: Vec<String>,
}

impl Parser {
   pub fn new(lexer: Lexer) -> Self {
      let mut p: Parser = Parser {
         lexer,
         cur_token: Token::new(TokenType::UNKNOWN, ""),
         peek_token: Token::new(TokenType::UNKNOWN, ""),
         errors: Vec::new(),
      };
      p.next_token();
      p.next_token();

      p
   }

   pub fn errors(&self) -> &Vec<String> {
      &self.errors
   } 

   pub fn parse_program(&mut self) -> Result<Program> {
      let mut program: Program = Program {
         statements: vec![],
      };

      while self.cur_token.token_type != TokenType::EOF {
         match self.parse_statement() {
            Some(statement) => program.statements.push(statement),
            None => {},
         }
         self.next_token();
      }

      Ok(program)
   }

   fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
      return match self.cur_token.token_type {
         TokenType::LET => {
            match self.parse_let_statement() {
               Some(let_stmt) => Some(Box::new(let_stmt)),
               None => None,
            }
         },
         TokenType::RETURN => {
            match self.parse_return_statement() {
               Some(return_stmt) => Some(Box::new(return_stmt)),
               None => None,
            }
         },
         _ => None,
      }
   }

   fn parse_let_statement(&mut self) -> Option<LetStatement> {
      let cur_token: Token = self.cur_token.clone();

      if !self.expect_peek(TokenType::IDENT) {
         return None; 
      }

      let name: Identifier = Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() };

      if !self.expect_peek(TokenType::ASSIGN) {
         return None;
      }

      // TODO: We're skipping the expressions until we encounter a semicolon
      while !self.cur_token_is(TokenType::SEMICOLON) {
         self.next_token();
      }
      
      // page 40
      // Value/Expression is None for now, change away from Option<> (after above TODO is solved)
      Some(LetStatement { token: cur_token, name, value: None })
   }

   fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
      let cur_token: Token = self.cur_token.clone();

      self.next_token();

      // TODO: We're skipping the expressions until we encounter a semicolon
      while !self.cur_token_is(TokenType::SEMICOLON) {
         self.next_token();
      }

      // Return_value/Expression is None for now, change away from Option<> (after above TODO is solved)
      Some(ReturnStatement { token: cur_token, return_value: None })
   }

   fn cur_token_is(&mut self, token_type: TokenType) -> bool {
      self.cur_token.token_type == token_type
   }

   fn peek_token_is(&mut self, token_type: TokenType) -> bool {
      self.peek_token.token_type == token_type
   }

   fn expect_peek(&mut self, token_type: TokenType) -> bool {
      if self.peek_token_is(token_type) {
         self.next_token();
         return true;
      } else {
         self.peek_error(token_type);
         return false;
      }
   }

   fn peek_error(&mut self, token_type: TokenType) {
      let msg: String = format!("Expected next token to be {:?}, got {:?} instead.", token_type, self.peek_token.token_type);
      self.errors.push(msg);
   }

   fn next_token(&mut self) -> Result<()> {
      self.cur_token = self.peek_token.clone();
      self.peek_token = match self.lexer.next_token() {
         Ok(tok) => tok,
         Err(e) => return Err(e)
      };
      Ok(())
   }


}