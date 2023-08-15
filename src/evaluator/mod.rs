pub mod builtins;

use std::collections::HashMap;

use crate::objects::environment::Environment;
use crate::parser::ast::{Node, Program, IntegerLiteral, ExpressionStatement, Statement, Expression, Boolean, PrefixExpression, InfixExpression, BlockStatement, IfExpression, ReturnStatement, LetStatement, Identifier, FunctionLiteral, CallExpression, StringLiteral, ArrayLiteral, IndexExpression, HashLiteral};
use crate::objects::{Object, Integer, Null, ObjectTypes, ReturnValue, Error, Function, MkyString, BuiltIn, Array, HashKey, HashPair, Hash};

use self::builtins::lookup_builtins;

pub const NULL: Null = Null{};
pub const TRUE: crate::objects::Boolean  = crate::objects::Boolean { value: true };
pub const FALSE: crate::objects::Boolean = crate::objects::Boolean { value: false };
fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
   if input {
      Box::new(TRUE)
   } else {
      Box::new(FALSE)
   }
}

// Match the current AST Node being evaluated and evaluate it; returning an Object trait object
pub fn eval(node: Box<&dyn Node>, env: &mut Environment) -> Option<Box<dyn Object>> {

   if node.node_as_any().is::<Program>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<Program>().unwrap().statements;
      return eval_program(statements_to_eval, env)
   }

   if node.node_as_any().is::<PrefixExpression>() {
      let node_to_eval: &PrefixExpression = node.node_as_any().downcast_ref::<PrefixExpression>().unwrap();
      let right: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node()), env);
      if is_error(right.as_ref()) {
         return right
      }
      
      return Some(eval_prefix_expression(node_to_eval.operator.clone(), right.unwrap()))
   }

   if node.node_as_any().is::<InfixExpression>() {
      let node_to_eval: &InfixExpression = node.node_as_any().downcast_ref::<InfixExpression>().unwrap();
      let left: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.left.as_ref().unwrap().as_node()), env);
      if is_error(left.as_ref()) {
         return left
      }
      let right: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.right.as_ref().unwrap().as_node()), env);
      if is_error(right.as_ref()) {
         return right
      }
      return Some(eval_infix_expression(node_to_eval.operator.clone(), left.unwrap(), right.unwrap()))
   }

   if node.node_as_any().is::<FunctionLiteral>() {
      let fn_node: &FunctionLiteral = node.node_as_any().downcast_ref::<FunctionLiteral>().unwrap();
      return Some(Box::new(Function { params: fn_node.params.clone(), body: fn_node.body.clone(), env: env.clone()}));
   }

   if node.node_as_any().is::<CallExpression>() {
      let ce_node: &CallExpression = node.node_as_any().downcast_ref::<CallExpression>().unwrap();
      let function: Option<Box<dyn Object>> = eval(Box::new(ce_node.function.as_ref().unwrap().as_node()), env);
      if is_error(function.as_ref()) {
         return function
      }

      let args: Vec<Box<dyn Object>> = eval_expressions(ce_node.arguments.as_ref().clone(), env);
      if args.len() == 1 && is_error(args.get(0)) {
         return Some(args.get(0).unwrap().clone())
      }
      return apply_function(function.unwrap(), args)
   }

   if node.node_as_any().is::<ExpressionStatement>() {
      let node_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ExpressionStatement>().unwrap().expression.as_ref().unwrap();
      return eval(Box::new(node_to_eval.as_node()), env)
   }

   if node.node_as_any().is::<Identifier>() {
      return eval_identifier(node.node_as_any().downcast_ref::<Identifier>().unwrap(), env)
   }

   if node.node_as_any().is::<HashLiteral>() {
      return eval_hash_literal(node.node_as_any().downcast_ref::<HashLiteral>().unwrap(), env)
   }

   if node.node_as_any().is::<StringLiteral>() {
      let node_to_eval: &StringLiteral = node.node_as_any().downcast_ref::<StringLiteral>().unwrap();
      return Some(Box::new(MkyString { value: node_to_eval.value.clone() }))
   }

   if node.node_as_any().is::<ArrayLiteral>() {
      let node_to_eval: &ArrayLiteral = node.node_as_any().downcast_ref::<ArrayLiteral>().unwrap();
      let elements: Vec<Box<dyn Object>> = eval_expressions(Some(&node_to_eval.elements), env);

      // Found an error while evaluating the elements, and 
      if elements.len() == 1 && is_error(elements.get(0)) {
         return Some(elements.get(0).unwrap().clone());
      }

      return Some(Box::new(Array { elements }))
   }

   if node.node_as_any().is::<LetStatement>() {
      let node_to_eval: &LetStatement = node.node_as_any().downcast_ref::<LetStatement>().unwrap();
      let value: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.value.as_ref().unwrap().as_node()), env);
      if is_error(value.as_ref()) {
         return value
      }
      env.set(&node_to_eval.name.value, value.unwrap());
   }

   if node.node_as_any().is::<ReturnStatement>() {
      let return_value_to_eval: &Box<dyn Expression> = node.node_as_any().downcast_ref::<ReturnStatement>().unwrap().return_value.as_ref().unwrap();
      let value: Option<Box<dyn Object>> = eval(Box::new(return_value_to_eval.as_node()), env);       
      if is_error(value.as_ref()) {
         return value
      }
      return Some(Box::new(ReturnValue { value: value.unwrap() }))
   }

   if node.node_as_any().is::<BlockStatement>() {
      let statements_to_eval: &Vec<Box<dyn Statement>> = &node.node_as_any().downcast_ref::<BlockStatement>().unwrap().statements;
      return eval_block_statement(statements_to_eval, env)
   }

   if node.node_as_any().is::<IfExpression>() {
      let if_expr_to_eval: &IfExpression = node.node_as_any().downcast_ref::<IfExpression>().unwrap();
      return Some(eval_if_expression(&if_expr_to_eval, env))
   }

   if node.node_as_any().is::<IndexExpression>() {
      let node_to_eval: &IndexExpression = node.node_as_any().downcast_ref::<IndexExpression>().unwrap();

      let left: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.left.as_node()), env);
      if is_error(left.as_ref()) {
         return left
      }

      let index: Option<Box<dyn Object>> = eval(Box::new(node_to_eval.index.as_node()), env);
      if is_error(index.as_ref()) {
         return index
      }
      
      return Some(eval_index_expression(left.unwrap(), index.unwrap()));
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
         
         if result.as_ref().unwrap().as_any().is::<Error>() {
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
   } else if left.r#type() == ObjectTypes::StringObj.to_string() && right.r#type() == ObjectTypes::StringObj.to_string() {
      return eval_string_infix_expression(operator, left, right);
   }
   
   Box::new(Error::new(format!("unknown operator: {} {} {}", left.r#type(), operator, right.r#type())))
}

fn eval_string_infix_expression(operator: String, left: Box<dyn Object>, right: Box<dyn Object>) -> Box<dyn Object> {
   if operator != "+" {
      return Box::new(Error::new(format!("unknown operator: {} {} {}", left.r#type(), operator, right.r#type())))
   }
   let left_val: String = left.as_any().downcast_ref::<MkyString>().unwrap().value.clone();
   let right_val: String = right.as_any().downcast_ref::<MkyString>().unwrap().value.clone();
   return Box::new(MkyString { value: format!("{}{}", left_val, right_val)})
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
   let condition: Option<Box<dyn Object>> = eval(Box::new(if_expr.condition.as_ref().unwrap().as_node()), env);
   if is_error(condition.as_ref()) {
      return condition.unwrap()
   }

   if is_truthy(condition.unwrap()) {
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
         if result.as_ref().unwrap().as_any().is::<ReturnValue>() {
            return result
         } 
         if result.as_ref().unwrap().as_any().is::<Error>() {
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
      Some(object) => Some(object.clone()),

      None => {
         match lookup_builtins(&node.value) {
            Some(builtin) => Some(Box::new(builtin)),
            None => Some(Box::new(Error::new(format!("identifier not found: {}", node.value))))
         }
      }
   }
}

fn eval_expressions(exps: Option<&Vec<Box<dyn Expression>>>, env: &mut Environment) -> Vec<Box<dyn Object>> {
   let mut result: Vec<Box<dyn Object>> = vec![];
   match exps {
      Some(exprs) => {
         for e in exprs {
            let eval: Option<Box<dyn Object>> = eval(Box::new(e.as_node()), env);
            if is_error(eval.as_ref()) {
               return vec![eval.unwrap()]
            }
            result.push(eval.unwrap());
         }
      },

      None => return result
   }
   result
}

fn apply_function(function: Box<dyn Object>, args: Vec<Box<dyn Object>>) -> Option<Box<dyn Object>> {
   if let Some(func) = function.as_any().downcast_ref::<Function>() {
      let mut extended_env: Environment = extend_function_env(func, args);
      let eval: Option<Box<dyn Object>> = eval(Box::new(func.body.as_ref().unwrap().as_node()), &mut extended_env);

      return unwrap_return_value(eval);
   } else if let Some(builtin_func) = function.as_any().downcast_ref::<BuiltIn>() {
      return Some((builtin_func.func)(args))
   } else {
      return Some(Box::new(Error::new(format!("not a function: {}", function.r#type()))))  
   }
}

fn extend_function_env(function: &Function, args: Vec<Box<dyn Object>>) -> Environment {
   let mut env: Environment = Environment::new_enclosed_env(function.env.clone());
   for (param_idx, param) in function.params.as_ref().unwrap().iter().enumerate() {
     env.set(&param.value, args.get(param_idx).unwrap().clone());
   }

   env
}

fn unwrap_return_value(obj: Option<Box<dyn Object>>) -> Option<Box<dyn Object>> {
   if obj.is_some() {
      let uw_obj: Box<dyn Object> = obj.unwrap();
      if let Some(return_value) = uw_obj.as_ref().as_any().downcast_ref::<ReturnValue>() {
         let val: &Box<dyn Object> = &return_value.value;
         return Some(val.to_owned())
      } else {
         return Some(uw_obj)
      }
   } else {
      None
   }
}

fn eval_index_expression(left: Box<dyn Object>, index: Box<dyn Object>) -> Box<dyn Object> {
   if left.r#type() == ObjectTypes::ArrayObj.to_string() && index.r#type() == ObjectTypes::IntegerObj.to_string() {
      return eval_array_index_expression(left, index)
   } else if left.r#type() == ObjectTypes::HashObj.to_string() {
      return eval_hash_index_expression(left, index)
   } else {
      return Box::new(Error::new(format!("index operator not supported: {}", left.r#type())))
   }
}

fn eval_array_index_expression(array: Box<dyn Object>, index: Box<dyn Object>) -> Box<dyn Object> {
   let array_obj: &Array = array.as_any().downcast_ref::<Array>().unwrap();
   let idx: i64 = index.as_any().downcast_ref::<Integer>().unwrap().value.clone();

   let max: i64 = array_obj.elements.len() as i64 - 1;
   if idx < 0 || idx > max {
      return Box::new(NULL)
   }

   return array_obj.elements.get(idx as usize).unwrap().clone()
}

fn eval_hash_index_expression(hash: Box<dyn Object>, index: Box<dyn Object>) -> Box<dyn Object> {
   let hash_obj: &Hash = hash.as_any().downcast_ref::<Hash>().unwrap();

   if !index.is_hashable() {
      return Box::new(Error::new(format!("unusable as hash key: {}", index.r#type())))
   }

   return match hash_obj.pairs.get(&index.downcast_hashable().unwrap().hash_key()) {
      Some(hp) => hp.value.clone(),
      None => Box::new(NULL)
   }
}

fn eval_hash_literal(node: &HashLiteral, env: &mut Environment) -> Option<Box<dyn Object>> {
   let mut pairs: HashMap<HashKey, HashPair> = HashMap::new();

   for (key_node, value_node) in &node.pairs {
      let key: Option<Box<dyn Object>> = eval(Box::new(key_node.as_node()), env);
      if is_error(key.as_ref()) {
         return key
      }
      
      let key: Box<dyn Object> = key.unwrap();
      if !key.is_hashable() {
         return Some(Box::new(Error::new(format!("unusable as hash key: {}", key.r#type()))))
      }
      
      let value: Option<Box<dyn Object>> = eval(Box::new(value_node.as_node()), env);
      if is_error(value.as_ref()) {
         return value
      }

      let hashed: HashKey = key.downcast_hashable().unwrap().hash_key();
      pairs.insert(hashed, HashPair { key, value: value.unwrap() });
   }

   return Some(Box::new(Hash { pairs }))
}
