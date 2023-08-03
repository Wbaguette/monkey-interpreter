#![allow(unused)]

use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement};
use crate::objects::{Object, Integer};

pub fn eval(node: &dyn Node) -> Option<Box<dyn Object>> {
   println!("{}", node.string());
   // Statements
   match node.node_as_any().downcast_ref::<Program>() {
      Some(p) => return eval_statements(&p.statements),
      None => {},
   }

   match node.node_as_any().downcast_ref::<ExpressionStatement>() {
      Some(ep) => return eval(ep),
      None => {},
   }
   
   // Expressions
   match node.node_as_any().downcast_ref::<IntegerLiteral>() {
      Some(il) => return Some(Box::new(Integer { value: il.value })),
      None => {},
   }

   None
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Option<Box<dyn Object>> {
   let mut result: Option<Box<dyn Object>> = None;

   for stmt in stmts {
      let thing: &dyn Node = stmt.as_node();
      result = match eval(thing) {
         Some(value) => {
            panic!("{:?}", value);
            Some(value)
         },
         None => result,
      };
      if result.is_some() {
         break;
      }
   }

   result
}