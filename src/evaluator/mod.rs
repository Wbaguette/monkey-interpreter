#![allow(unused)]

use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement, Expression};
use crate::objects::{Object, Integer};

pub fn eval(node: Box<&dyn Node>) -> Option<Box<dyn Object>> {

   if node.node_as_any().is::<Program>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<Program>().unwrap().statements;
      return eval_statements(statements_to_eval)
   }

   if node.node_as_any().is::<ExpressionStatement>() {
      let node_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ExpressionStatement>().unwrap().expression.as_ref().unwrap();
      return eval(Box::new(node_to_eval.as_node()))
   }

   if node.node_as_any().is::<IntegerLiteral>() {
      let integer_literal_node = node.node_as_any().downcast_ref::<IntegerLiteral>().unwrap();
      return Some(Box::new(Integer { value: integer_literal_node.value }))
   }

   None
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Option<Box<dyn Object>> {
   let mut result: Option<Box<dyn Object>> = None;

   for stmt in stmts {
      let thing: Box<&dyn Node> = Box::new(stmt.as_node());
      result = match eval(thing) {
         Some(value) => {
            Some(value)
         },
         None => result,
      };
      if result.is_some() {
         break;
      }
   }

   return result
}