#![allow(unused)]

use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement, Expression, Boolean, PrefixExpression, InfixExpression, BlockStatement, IfExpression};
use crate::objects::{Object, Integer, Null, ObjectTypes};

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

   if node.node_as_any().is::<InfixExpression>() {
      let node_to_eval: &InfixExpression = node.node_as_any().downcast_ref::<InfixExpression>().unwrap();
      let left: Box<dyn Object> = eval(Box::new(node_to_eval.left.as_ref().unwrap().as_node())).unwrap();
      let right: Box<dyn Object> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node())).unwrap();
      return Some(eval_infix_expression(node_to_eval.operator.clone(), left, right))
   }

   if node.node_as_any().is::<ExpressionStatement>() {
      let node_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ExpressionStatement>().unwrap().expression.as_ref().unwrap();
      return eval(Box::new(node_to_eval.as_node()))
   }

   if node.node_as_any().is::<BlockStatement>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<BlockStatement>().unwrap().statements;
      return eval_statements(statements_to_eval)
   }

   if node.node_as_any().is::<IfExpression>() {
      let if_expr_to_eval: &IfExpression = node.node_as_any().downcast_ref::<IfExpression>().unwrap();
      return Some(eval_if_expression(&if_expr_to_eval))
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




// Helper eval functions




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
      _ => Box::new(NULL)
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
      return Box::new(NULL)
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
   }else {
      return Box::new(NULL)
   }
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

      _ => Box::new(NULL)
   }
}

fn eval_if_expression(if_expr: &IfExpression) -> Box<dyn Object> {
   let condition: Box<dyn Object> = eval(Box::new(if_expr.condition.as_ref().unwrap().as_node())).unwrap();

   if is_truthy(condition) {
      return eval(Box::new(if_expr.consequence.as_ref().unwrap().as_node())).unwrap()
   } else if if_expr.alternative.is_some() {
      return eval(Box::new(if_expr.alternative.as_ref().unwrap().as_node())).unwrap()
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