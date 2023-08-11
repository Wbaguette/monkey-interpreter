#![allow(unused)]

use core::panic;
use std::borrow::BorrowMut;

use crate::objects::environment::Environment;
use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement, Expression, Boolean, PrefixExpression, InfixExpression, BlockStatement, IfExpression, ReturnStatement, LetStatement, Identifier};
use crate::objects::{Object, Integer, Null, ObjectTypes, ReturnValue, Error, match_object_to};

pub const NULL: Null = Null{};
const TRUE: crate::objects::Boolean  = crate::objects::Boolean { value: true };
const FALSE: crate::objects::Boolean = crate::objects::Boolean { value: false };
fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
   if input {
      Box::new(TRUE)
   } else {
      Box::new(FALSE)
   }
}

pub fn eval(node: Box<&dyn Node>, env: &mut Environment) -> Option<Box<dyn Object>> {

   if node.node_as_any().is::<Program>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<Program>().unwrap().statements;
      return eval_program(statements_to_eval, env)
   }

   if node.node_as_any().is::<PrefixExpression>() {
      let node_to_eval: &PrefixExpression = node.node_as_any().downcast_ref::<PrefixExpression>().unwrap();
      let right: Box<dyn Object> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node()), env).unwrap();
      if is_error(Some(&right)) {
         return Some(right)
      }
      return Some(eval_prefix_expression(node_to_eval.operator.clone(), right))
   }

   if node.node_as_any().is::<InfixExpression>() {
      let node_to_eval: &InfixExpression = node.node_as_any().downcast_ref::<InfixExpression>().unwrap();
      let left: Box<dyn Object> = eval(Box::new(node_to_eval.left.as_ref().unwrap().as_node()), env).unwrap();
      if is_error(Some(&left)) {
         return Some(left)
      }
      let right: Box<dyn Object> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node()), env).unwrap();
      if is_error(Some(&right)) {
         return Some(right)
      }
      return Some(eval_infix_expression(node_to_eval.operator.clone(), left, right))
   }

   if node.node_as_any().is::<ExpressionStatement>() {
      let node_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ExpressionStatement>().unwrap().expression.as_ref().unwrap();
      return eval(Box::new(node_to_eval.as_node()), env)
   }

   if node.node_as_any().is::<Identifier>() {
      return eval_identifier(node.node_as_any().downcast_ref::<Identifier>().unwrap(), env)
   }

   if node.node_as_any().is::<LetStatement>() {
      let node_to_eval: &LetStatement = node.node_as_any().downcast_ref::<LetStatement>().unwrap();
      let value: Box<dyn Object> = eval(Box::new(node_to_eval.value.as_ref().unwrap().as_node()), env).unwrap();
      if is_error(Some(&value)) {
         return Some(value)
      }
      env.set(&node_to_eval.name.value, value);
   }

   if node.node_as_any().is::<ReturnStatement>() {
      let return_value_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ReturnStatement>().unwrap().return_value.as_ref().unwrap();
      let value: Box<dyn Object> = eval(Box::new(return_value_to_eval.as_node()), env).unwrap();       
      if is_error(Some(&value)) {
         return Some(value)
      }
      return Some(Box::new(ReturnValue { value }))
   }

   if node.node_as_any().is::<BlockStatement>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<BlockStatement>().unwrap().statements;
      return eval_block_statement(statements_to_eval, env)
   }

   if node.node_as_any().is::<IfExpression>() {
      let if_expr_to_eval: &IfExpression = node.node_as_any().downcast_ref::<IfExpression>().unwrap();
      return Some(eval_if_expression(&if_expr_to_eval, env))
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

   // Hopefully we dont ever get here ^_^
   None
}




// Helper eval functions




fn eval_program(stmts: &Vec<Box<dyn Statement>>, env: &mut Environment) -> Option<Box<dyn Object>> {
   let mut result: Option<Box<dyn Object>> = None;

   for stmt in stmts {
      let thing: Box<&dyn Node> = Box::new(stmt.as_node());
      result = eval(thing, env);

      if result.is_some() {
         if let Some(result_value) = result.as_ref().unwrap().as_any().downcast_ref::<ReturnValue>() {
            let val: &Box<dyn Object> = &result_value.value;

            if val.as_any().is::<Integer>() {
               return Some(Box::new(*val.as_any().downcast_ref::<Integer>().unwrap()))
            }

            if val.as_any().is::<crate::objects::Boolean>() {
               return Some(Box::new(*val.as_any().downcast_ref::<crate::objects::Boolean>().unwrap()))
            }

            if val.as_any().is::<Null>() {
               return Some(Box::new(*val.as_any().downcast_ref::<Null>().unwrap()))
            }
         }
      
         if let Some(error_value) = result.as_ref().unwrap().as_any().downcast_ref::<Error>() {
            return result
         }
      }
   }

   result
}

fn eval_prefix_expression(operator: String, right: Box<dyn Object>) -> Box<dyn Object> {
   return match operator.as_str() {
      "!" => eval_bang_operator_expression(right),
      "-" => eval_minus_prefix_expression(right),
      _ => Box::new(Error::new(format!("unknown operator: {}{}", operator, right.r#type())))                         
   }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
   if right.as_any().is::<crate::objects::Boolean>() {
      return match right.as_any().downcast_ref::<crate::objects::Boolean>().unwrap().value {
         true => Box::new(FALSE),
         false => Box::new(TRUE)
      }
   } else if right.as_any().is::<Null>() {
      return Box::new(TRUE)
   }

   // DEFAULT IS TO RETURN FALSE
   Box::new(FALSE)
}

fn eval_minus_prefix_expression(right: Box<dyn Object>) -> Box<dyn Object> {
   if right.r#type() != ObjectTypes::IntegerObj.to_string() {
      return Box::new(Error::new(format!("unknown operator: -{}", right.r#type())))      
   } 

   let value: i64 = right.as_any().downcast_ref::<Integer>().unwrap().value;
   Box::new(Integer { value: -value })
}

fn eval_infix_expression(operator: String, left: Box<dyn Object>, right: Box<dyn Object>) -> Box<dyn Object> {
   if left.r#type() == ObjectTypes::IntegerObj.to_string() && right.r#type() == ObjectTypes::IntegerObj.to_string() {
      return eval_integer_infix_expression(operator, left, right)
   } else if operator == "==" {
      return native_bool_to_boolean_object(left.inspect() == right.inspect())
   } else if operator == "!=" {
      return native_bool_to_boolean_object(left.inspect() != right.inspect())
   } else if left.r#type() != right.r#type() {
      return Box::new(Error::new(format!("type mismatch: {} {} {}", left.r#type(), operator, right.r#type())))
   } 
   
   Box::new(Error::new(format!("unknown operator: {} {} {}", left.r#type(), operator, right.r#type())))
}

fn eval_integer_infix_expression(operator: String, left: Box<dyn Object>, right: Box<dyn Object>) -> Box<dyn Object> {
   let left_val: i64 = left.as_any().downcast_ref::<Integer>().unwrap().value;
   let right_val: i64 = right.as_any().downcast_ref::<Integer>().unwrap().value;

   return match operator.as_str() {
      "+" => Box::new(Integer { value: left_val + right_val }),
      "-" => Box::new(Integer { value: left_val - right_val }),
      "*" => Box::new(Integer { value: left_val * right_val }),
      "/" => Box::new(Integer { value: left_val / right_val }),

      "<" => native_bool_to_boolean_object(left_val < right_val),
      ">" => native_bool_to_boolean_object(left_val > right_val),
      "==" => native_bool_to_boolean_object(left_val == right_val),
      "!=" => native_bool_to_boolean_object(left_val != right_val),
 
      _ => Box::new(Error::new(format!("unknown operator: {} {} {}", left.r#type(), operator, right.r#type())))
   }
}

fn eval_if_expression(if_expr: &IfExpression, env: &mut Environment) -> Box<dyn Object> {
   let condition: Box<dyn Object> = eval(Box::new(if_expr.condition.as_ref().unwrap().as_node()), env).unwrap();
   if is_error(Some(&condition)) {
      return condition
   }

   if is_truthy(condition) {
      return eval(Box::new(if_expr.consequence.as_ref().unwrap().as_node()), env).unwrap()
   } else if if_expr.alternative.is_some() {
      return eval(Box::new(if_expr.alternative.as_ref().unwrap().as_node()), env).unwrap()
   } else {
      return Box::new(NULL)
   }
}

fn is_truthy(condition: Box<dyn Object>) -> bool {
   if condition.as_any().is::<Null>() {
      return false
   }
   if condition.as_any().is::<crate::objects::Boolean>() {
      let c: &crate::objects::Boolean = condition.as_any().downcast_ref::<crate::objects::Boolean>().unwrap();
      return c.value
   }

   true
}

fn eval_block_statement(stmts: &Vec<Box<dyn Statement>>, env: &mut Environment) -> Option<Box<dyn Object>> {
   let mut result: Option<Box<dyn Object>> = None;

   for stmt in stmts {
      let thing: Box<&dyn Node> = Box::new(stmt.as_node());
      result = eval(thing, env);

      if result.is_some() {
         if let Some(result_value) = result.as_ref().unwrap().as_any().downcast_ref::<ReturnValue>() {
            return result
         } 
         if let Some(error_value) = result.as_ref().unwrap().as_any().downcast_ref::<Error>() {
            return result
         }
      }
   }

   result
}

fn is_error(obj: Option<&Box<dyn Object>>) -> bool {
   if obj.is_some() {
      return obj.unwrap().r#type() == ObjectTypes::ErrorObj.to_string()
   }
   false
}

fn eval_identifier(node: &Identifier, env: &Environment) -> Option<Box<dyn Object>> {
   return match env.get(&node.value) {
      Some(object) => {
         let x = match_object_to(object);
         Some(x)
      },
      None => Some(Box::new(Error::new(format!("identifier not found: {}", node.value))))
   }
}
