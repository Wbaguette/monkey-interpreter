#![allow(unused)]

pub mod ast;

use std::vec;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::parser::ast::{Program, Identifier, LetStatement, Statement, ReturnStatement, PrefixExpression, InfixExpression, Boolean};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{Expression, ExpressionStatement, IntegerLiteral};
use color_eyre::Result;

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
   LOWEST = 1,
   EQUALS,          // ==
   LESSGREATER,     // < or >
   SUM,             // +
   PRODUCT,         // *
   PREFIX,          // -X or !X
   CALL,            // myFunction(X)   
}

lazy_static! {
   static ref PRECEDENCES: HashMap<TokenType, Precedence> = {
      let mut map = HashMap::new();
      map.insert(TokenType::EQ, Precedence::EQUALS);
      map.insert(TokenType::NOTEQ, Precedence::EQUALS);
      map.insert(TokenType::LT, Precedence::LESSGREATER);
      map.insert(TokenType::GT, Precedence::LESSGREATER);
      map.insert(TokenType::PLUS, Precedence::SUM);
      map.insert(TokenType::MINUS, Precedence::SUM);
      map.insert(TokenType::SLASH, Precedence::PRODUCT);
      map.insert(TokenType::ASTERISK, Precedence::PRODUCT);

      map
   };
}


#[derive(Debug, Clone)]
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
      p.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
      p.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
      p.register_prefix(TokenType::TRUE, Parser::parse_boolean);
      p.register_prefix(TokenType::FALSE, Parser::parse_boolean);

      p.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
      p.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
      p.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
      p.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
      p.register_infix(TokenType::EQ, Parser::parse_infix_expression);
      p.register_infix(TokenType::NOTEQ, Parser::parse_infix_expression);
      p.register_infix(TokenType::LT, Parser::parse_infix_expression);
      p.register_infix(TokenType::GT, Parser::parse_infix_expression);

      p
   }

   fn next_token(&mut self) -> Result<()> {
      self.cur_token = self.peek_token.clone();
      self.peek_token = match self.lexer.next_token() {
         Ok(tok) => tok,
         Err(e) => return Err(e)
      };
      Ok(())
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







   // PARSING FUNCTIONS

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
         None => {
            self.no_prefix_parse_error(self.cur_token.token_type);
            return None
         },
      };

      let mut left_exp: Box<dyn Expression> = prefix_fn(self).unwrap();

      while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
         let infix_fn: &fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>> = match self.infix_parse_fns.get(&self.peek_token.token_type) {
            Some(infix_fn) => infix_fn,
            None => {
               return Some(left_exp);
            }
         };
         
         let ifx: fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>> = *infix_fn;
         self.next_token();
         left_exp = ifx(self, left_exp).unwrap(); 
      }

      Some(left_exp)
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

   fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
      let cur_token: Token = self.cur_token.clone();
      let operator: String = self.cur_token.literal.clone();
      
      self.next_token();

      Some(Box::new(PrefixExpression { token: cur_token, operator, right: self.parse_expression(Precedence::PREFIX) }))
   }

   fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
      let mut expr: InfixExpression = InfixExpression { 
         token: self.cur_token.clone(), left: Some(left), operator: self.cur_token.literal.clone(), right: None 
      };

      let precedence: Precedence = self.cur_precedence();
      self.next_token();
      expr.right = self.parse_expression(precedence);

      Some(Box::new(expr))
   }

   fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
      Some(Box::new(Boolean {
         token: self.cur_token.clone(),
         value: self.cur_token_is(TokenType::TRUE),
      }))
   }






   // HELPER FUNCTIONS 

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

   fn no_prefix_parse_error(&mut self, token_type: TokenType) {
      let msg: String = format!("No prefix parse function for {:?} found.", token_type);
      self.errors.push(msg);
   }

   fn peek_precedence(&mut self) -> Precedence {
      return match PRECEDENCES.get(&self.peek_token.token_type) {
         Some(p) => p.clone(),
         None => Precedence::LOWEST,
      };
   }

   fn cur_precedence(&mut self) -> Precedence {
      return match PRECEDENCES.get(&self.cur_token.token_type) {
         Some(p) => p.clone(),
         None => Precedence::LOWEST,
      };
   }

}