#![allow(unused)]

use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement, Expression, Boolean, PrefixExpression};
use crate::objects::{Object, Integer, Null, ObjectTypes};

// Use these instead of creating new instances every time...
const NULL: Null = Null{};

const TRUE: crate::objects::Boolean  = crate::objects::Boolean { value: true };
const FALSE: crate::objects::Boolean = crate::objects::Boolean { value: false };
fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
   if input {
      Box::new(TRUE)
   } else {
      Box::new(FALSE)
   }
}

pub fn eval(node: Box<&dyn Node>) -> Option<Box<dyn Object>> {

   if node.node_as_any().is::<Program>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<Program>().unwrap().statements;
      return eval_statements(statements_to_eval)
   }

   if node.node_as_any().is::<PrefixExpression>() {
      let node_to_eval: &PrefixExpression = node.node_as_any().downcast_ref::<PrefixExpression>().unwrap();
      let right: Box<dyn Object> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node())).unwrap();
      return Some(eval_prefix_expression(node_to_eval.operator.clone(), right))
   }

   if node.node_as_any().is::<ExpressionStatement>() {
      let node_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ExpressionStatement>().unwrap().expression.as_ref().unwrap();
      return eval(Box::new(node_to_eval.as_node()))
   }

   if node.node_as_any().is::<Boolean>() {
      let boolean_node: &Boolean = node.node_as_any().downcast_ref::<Boolean>().unwrap();
      // Just use an already created TRUE and FALSE Boolean Object 
      return Some(native_bool_to_boolean_object(boolean_node.value))
   }

   if node.node_as_any().is::<IntegerLiteral>() {
      let integer_literal_node: &IntegerLiteral = node.node_as_any().downcast_ref::<IntegerLiteral>().unwrap();
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

fn eval_prefix_expression(operator: String, right: Box<dyn Object>) -> Box<dyn Object> {
   return match operator.as_str() {
      "!" => eval_bang_operator_expression(right),
      "-" => eval_minus_prefix_expression(right),
      _ => Box::new(NULL),
   }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
   if right.as_any().is::<crate::objects::Boolean>() {
      return match right.as_any().downcast_ref::<crate::objects::Boolean>().unwrap().value {
         true => Box::new(FALSE),
         false => Box::new(TRUE),
      }
   } else if right.as_any().is::<Null>() {
      return Box::new(TRUE)
   }

   // DEFAULT IS TO RETURN FALSE
   Box::new(FALSE)
}

fn eval_minus_prefix_expression(right: Box<dyn Object>) -> Box<dyn Object> {
   if right.r#type() != ObjectTypes::IntegerObj.to_string() {
      return Box::new(NULL)
   } 

   let value: i64 = right.as_any().downcast_ref::<Integer>().unwrap().value;
   Box::new(Integer { value: -value })
}