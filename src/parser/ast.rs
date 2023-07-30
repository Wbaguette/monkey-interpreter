#![allow(unused)]

use crate::lexer::token::Token;
use std::any::Any;

pub trait Node {
   fn token_literal(&self) -> &str;
   fn string(&self) -> String;
}

pub trait Statement: Node + Any {
   fn statement_node(&self);
   fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Any {
   fn expression_node(&self);
   fn as_any(&self) -> &dyn Any;
}
impl std::fmt::Debug for dyn Expression {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(format!("{}", self.string()).as_str())
   }
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

   fn string(&self) -> String {
      let mut out: String = String::new();

      for s in &self.statements {
         out.push_str(s.string().as_str());
      }
      out
   }
}



#[derive(Debug)]
pub struct Identifier {
   pub token: Token,  // this should always be TokenType::IDENT   (binding)
   pub value: String,
}
impl Node for Identifier {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.value.clone()
   }

}
impl Expression for Identifier {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}





#[derive(Debug)]
pub struct LetStatement {
   pub token: Token,   // this should always be TokenType::LET
   pub name: Identifier,   // LetStatement.name.token_literal() should return the binding value: let foo = 5;   => "foo"
   pub value: Option<Box<dyn Expression>>,
}
impl Node for LetStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("{} ", self.token_literal()).as_str());
      out.push_str(format!("{} = ", self.name.string()).as_str());

      if self.value.is_some() {
         out.push_str(self.value.as_ref().unwrap().string().as_str());
      }

      out.push_str(";");

      out
   }
}
impl Statement for LetStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}



#[derive(Debug)]
pub struct ReturnStatement {
   pub token: Token,
   pub return_value: Option<Box<dyn Expression>>,
}
impl Node for ReturnStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("{} ", self.token_literal()).as_str());

      if self.return_value.is_some() {
         out.push_str(self.return_value.as_ref().unwrap().string().as_str());
      }

      out.push_str(";");

      out
   }
}
impl Statement for ReturnStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}



#[derive(Debug)]
pub struct ExpressionStatement {
   pub token: Token,
   pub expression: Option<Box<dyn Expression>>,
}
impl Node for ExpressionStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      if self.expression.is_some() {
         out.push_str(self.expression.as_ref().unwrap().string().as_str());
      }
      out
   }
}
impl Statement for ExpressionStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}



#[derive(Debug)]
pub struct IntegerLiteral {
   pub token: Token,
   pub value: i64,
}
impl Node for IntegerLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.token.literal.clone()
   }
}
impl Expression for IntegerLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}



#[derive(Debug)]
pub struct PrefixExpression {
   pub token: Token,
   pub operator: String,
   pub right: Option<Box<dyn Expression>>,
}
impl Node for PrefixExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("({}", self.operator).as_str());
      out.push_str(format!("{})", self.right.as_ref().unwrap().string()).as_str());

      out
   }
}
impl Expression for PrefixExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
}