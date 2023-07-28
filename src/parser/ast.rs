#![allow(unused)]

use crate::lexer::token::Token;
use std::any::Any;

pub trait Node {
   fn token_literal(&self) -> &str;
}

pub trait Statement: Node + Any {
   fn statement_node(&self);
   fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
   fn expression_node(&self);
}

pub struct Program {
   pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
   fn token_literal(&self) -> &str {
      if self.statements.len() > 0 {
         return self.statements.get(0).unwrap().token_literal()
      } else {
         ""
      }
   }
}

pub struct Identifier {
   pub token: Token,  // this should always be TokenType::IDENT   (binding)
   pub value: String,
}

impl Node for Identifier {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }
}

impl Expression for Identifier {
   fn expression_node(&self) {}
}






pub struct LetStatement {
   pub token: Token,   // this should always be TokenType::LET
   pub name: Identifier,   // LetStatement.name.token_literal() should return the binding value: let foo = 5;   => "foo"
   pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }
}

impl Statement for LetStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}



pub struct ReturnStatement {
   pub token: Token,
   pub return_value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }
}

impl Statement for ReturnStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}