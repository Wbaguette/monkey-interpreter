#![allow(unused)]

pub mod ast;

use std::vec;
use std::collections::HashMap;

use crate::parser::ast::{Program, Identifier, LetStatement, Statement, ReturnStatement};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{Expression, ExpressionStatement, IntegerLiteral};
use color_eyre::Result;

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

#[derive(Debug)]
pub enum Precedence {
   LOWEST = 1,
   EQUALS,          // ==
   LESSGREATER,     // < or >
   SUM,             // +
   PRODUCT,         // *
   PREFIX,          // -X or !X
   CALL,            // myFunction(X)   
}


#[derive(Debug)]
pub struct Parser {
   lexer: Lexer,
   cur_token: Token,
   peek_token: Token,
   pub errors: Vec<String>,

   pub prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
   pub infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
   pub fn new(lexer: Lexer) -> Self {
      let mut p: Parser = Parser {
         lexer,
         cur_token: Token::new(TokenType::UNKNOWN, ""),
         peek_token: Token::new(TokenType::UNKNOWN, ""),
         errors: Vec::new(),
         prefix_parse_fns: HashMap::new(),
         infix_parse_fns: HashMap::new(),
      };
      // Move the jawn forward to be in position to parse
      p.next_token();
      p.next_token();

      // Register some functions for parsing
      p.register_prefix(TokenType::IDENT, Parser::parse_identifier);
      p.register_prefix(TokenType::INT, Parser::parse_integer_literal);

      p
   }

   fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
      self.prefix_parse_fns.insert(token_type, func);
   }

   fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
      self.infix_parse_fns.insert(token_type, func);  
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
         _ => {
            match self.parse_expression_statement() {
               Some(expr_stmt) => Some(Box::new(expr_stmt)),
               None => None,
            }
         },
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

   fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
      let expr_stmt: ExpressionStatement = ExpressionStatement {
         token: self.cur_token.clone(),
         expression: match self.parse_expression(Precedence::LOWEST) {
            Some(expr) => Some(expr),
            None => None,
         },
      };
      
      if self.peek_token_is(TokenType::SEMICOLON) {
         self.next_token();
      }

      Some(expr_stmt)
   }

   fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
      let prefix_fn: &fn(&mut Parser) -> Option<Box<dyn Expression>> = match self.prefix_parse_fns.get(&self.cur_token.token_type) {
         Some(prefix_fn) => prefix_fn,
         None => return None,
      };

      prefix_fn(self)
   }

   fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
      Some(Box::new(Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() }))
   }

   fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
      let cur_token: Token = self.cur_token.clone();

      let val: i64 = match self.cur_token.literal.parse::<i64>() {
         Ok(num) => num,
         Err(e) => {
            let msg: String = format!("Could not parse {} as i64. Error: {}", self.cur_token.literal, e);
            self.errors.push(msg);
            return None;
         },
      };

      Some(Box::new(IntegerLiteral { token: cur_token, value: val }))
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