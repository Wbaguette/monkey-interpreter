#![allow(unused)]

use std::any::Any;

use crate::objects::environment::Environment;
use crate::objects::{Integer, Boolean, Object, Null, Error, ReturnValue, Function, MkyString};
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::parser::ast::{Program, Node};
use crate::evaluator::{eval, self, NULL};

#[cfg(test)]

use crate::parser::{};




// SOME STRUCTS FOR CLEANER TESTING (AT LEAST I THINK SO...)





#[allow(non_camel_case_types)]
struct i64Test {
   input: String,
   expected: i64,
}
impl i64Test {
   pub fn new(input: &str, expected: i64) -> Self {
      i64Test { input: input.to_string(), expected }
   }

   pub fn test_me(&self) {
      match test_eval(self.input.clone()) {
         Some(eval) => test_integer_object(eval, self.expected),
         None => panic!("test_eval returned None")
      }
   }
}

struct BoolTest {
   input: String,
   expected: bool,
}
impl BoolTest {
   pub fn new(input: &str, expected: bool) -> Self {
      BoolTest { input: input.to_string(), expected }
   }

   pub fn test_me(&self) {
      match test_eval(self.input.clone()) {
         Some(eval) => test_boolean_object(eval, self.expected),
         None => panic!("test_eval returned None")
      }
   }
}

struct IfElseTest {
   input: String,
   expected: Option<i64>
}
impl IfElseTest {
   pub fn new(input: &str, expected: Option<i64>) -> Self {
      IfElseTest { input: input.to_string(), expected }
   }

   pub fn test_me(&self) {
      match test_eval(self.input.clone()) {
         Some(eval) => {
            if self.expected.is_some() {
               test_integer_object(eval, self.expected.unwrap())
            } else {
               test_null_object(eval)
            }
         }, 
         None => panic!("test_eval returned None.")
      }
   }
}

struct ErrorMessageTest {
   input: String,
   expected_msg: String,
}
impl ErrorMessageTest {
   pub fn new(input: &str, expected_msg: &str) -> Self {
      ErrorMessageTest { input: input.to_string(), expected_msg: expected_msg.to_string() }
   }

   pub fn test_me(&self) {
      match test_eval(self.input.clone()) {
         Some(eval) => {
            if let Some(error_object) = eval.as_any().downcast_ref::<Error>() {
               assert_eq!(self.expected_msg, error_object.message)
            } else {
               panic!("No error object returned. Got {:?}", eval)
            }
         },
         None => panic!("test_eval returned None.")
      }
   }
}




// HELPER FUNCTIONS




fn test_eval(input: String) -> Option<Box<dyn Object>> {
   let lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(p) => p,
      Err(e) => panic!("{}", e),
   };
   let mut env: Environment = Environment::new();

   return eval(Box::new(&program), &mut env)
}


fn test_integer_object(obj: Box<dyn Object>, expected: i64) {
   if let Some(result) = obj.as_any().downcast_ref::<Integer>() {
      assert_eq!(result.value, expected);
   } else {
      panic!("obj passed is not an Integer object.")
   }
}

fn test_boolean_object(obj: Box<dyn Object>, expected: bool) {
   if let Some(result) = obj.as_any().downcast_ref::<Boolean>() {
      assert_eq!(result.value, expected);
   } else {
      panic!("obj passed is not a Boolean object.")
   }
}

fn test_null_object(obj: Box<dyn Object>) {
   // This might be iffy with references... But the object passed here should always be a Null object 
   assert_eq!(&NULL, obj.as_any().downcast_ref::<Null>().unwrap());
}




// ACTUAL TEST FUNCTIONS




#[test]
fn test_eval_integer_expression() {
   i64Test::new("5", 5).test_me();
   i64Test::new("10", 10).test_me();
   i64Test::new("-5", -5).test_me();
   i64Test::new("-10", -10).test_me();

   i64Test::new("5 + 5 + 5 + 5 - 10", 10).test_me();
   i64Test::new("2 * 2 * 2 * 2 * 2", 32).test_me();
   i64Test::new("-50 + 100 + -50", 0).test_me();
   i64Test::new("5 * 2 + 10", 20).test_me();
   i64Test::new("5 + 2 * 10", 25).test_me();
   i64Test::new("20 + 2 * -10", 0).test_me();
   i64Test::new("50 / 2 * 2 + 10", 60).test_me();
   i64Test::new("2 * (5 + 10)", 30).test_me();
   i64Test::new("3 * 3 * 3 + 10", 37).test_me();
   i64Test::new("3 * (3 * 3) + 10", 37).test_me();
   i64Test::new("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50).test_me();
}

#[test]
fn test_eval_boolean_expression() {
   BoolTest::new("true", true).test_me();
   BoolTest::new("false", false).test_me();
   BoolTest::new("1 < 2", true).test_me();
   BoolTest::new("1 > 2", false).test_me();
   BoolTest::new("1 < 1", false).test_me();
   BoolTest::new("1 > 1", false).test_me();
   BoolTest::new("1 == 1", true).test_me();
   BoolTest::new("1 != 1", false).test_me();
   BoolTest::new("1 == 2", false).test_me();
   BoolTest::new("1 != 2", true).test_me();
   BoolTest::new("true == true", true).test_me();
   BoolTest::new("false == false", true).test_me();
   BoolTest::new("true == false", false).test_me();
   BoolTest::new("true != false", true).test_me();
   BoolTest::new("false != true", true).test_me();
   BoolTest::new("(1 < 2) == true", true).test_me();
   BoolTest::new("(1 < 2) == false", false).test_me();
   BoolTest::new("(1 > 2) == true", false).test_me();
   BoolTest::new("(1 > 2) == false", true).test_me();
}

#[test]
fn test_bang_operator() {
   BoolTest::new("!false", true).test_me();
   BoolTest::new("!true", false).test_me();
   BoolTest::new("!5", false).test_me();
   BoolTest::new("!!5", true).test_me();
   BoolTest::new("!!true", true).test_me();
   BoolTest::new("!!false", false).test_me();
}

#[test]
fn test_if_else_expressions() {
   IfElseTest::new("if (true) { 10 }", Some(10)).test_me();
   IfElseTest::new("if (false) { 10 }", None).test_me();
   IfElseTest::new("if (1) { 10 }", Some(10)).test_me();
   IfElseTest::new("if (1 < 2) { 10 }", Some(10)).test_me();
   IfElseTest::new("if (1 > 2) { 10 }", None).test_me();
   IfElseTest::new("if (1 > 2) { 10 } else { 20 }", Some(20)).test_me();
   IfElseTest::new("if (1 < 2) { 10 } else { 20 }", Some(10)).test_me();
}

#[test]
fn test_return_statements() {
   i64Test::new("return 10;", 10).test_me();
   i64Test::new("return 10; 9;", 10).test_me();
   i64Test::new("return 2 * 5; 9;", 10).test_me();
   i64Test::new("11; return 2 * 5; 9;", 10).test_me();
   
   i64Test::new("
      if (10 > 1) {
         if (10 > 1) {
            return 10;
         }

         return 1;
      }
      ", 10).test_me();
}

#[test]
fn test_error_handling() {
   ErrorMessageTest::new("5 + true;", "type mismatch: INTEGER + BOOLEAN").test_me();
   ErrorMessageTest::new("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN").test_me();
   ErrorMessageTest::new("-true", "unknown operator: -BOOLEAN").test_me();
   ErrorMessageTest::new("true + false;", "unknown operator: BOOLEAN + BOOLEAN").test_me();
   ErrorMessageTest::new("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN").test_me();
   ErrorMessageTest::new("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN").test_me();

   ErrorMessageTest::new("
      if (10 > 1) {
         if (10 > 1) {
            return true + false;
         }

         return 1;
      }
      ",
      "unknown operator: BOOLEAN + BOOLEAN").test_me();

   ErrorMessageTest::new("foobar", "identifier not found: foobar").test_me();
}

#[test]
fn test_let_statements() {
   i64Test::new("let a = 5; a;", 5).test_me();
   i64Test::new("let a = 5 * 5; a;", 25).test_me();
   i64Test::new("let a = 5; let b = a; let c = a + b + 5; c;", 15).test_me();
   i64Test::new("let a = 5; let b = a; let c = a + b + 5; c;", 15).test_me();
}

#[test]
fn test_function_object() {
   let input: String = String::from("fn(x) { x + 2; };");
   match test_eval(input) {
      Some(eval) => {
         if let Some(fn_obj) = eval.as_any().downcast_ref::<Function>() {
            assert_eq!(fn_obj.params.as_ref().unwrap().len(), 1);
            assert_eq!(fn_obj.params.as_ref().unwrap().get(0).unwrap().string(), "x");

            let expected_body: String = String::from("(x + 2)");
            assert_eq!(fn_obj.body.as_ref().unwrap().string(), expected_body);
         } else {
            panic!("'Object' trait object returned is not a Function")
         }
      }
      None => panic!("test_eval returned None")
   }
}

#[test]
fn test_function_application() {
   i64Test::new("let identity = fn(x) { x; }; identity(5);", 5).test_me();
   i64Test::new("let identity = fn(x) { return x; }; identity(5);", 5).test_me();
   i64Test::new("let double = fn(x) { x * 2; }; double(5);", 10).test_me();
   i64Test::new("let add = fn(x, y) { x + y; }; add(5, 5);", 10).test_me();
   i64Test::new("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20).test_me();
   i64Test::new("fn(x) { x; }(5)", 5).test_me();
}

#[test]
fn test_closures() {
   i64Test::new("
      let newAdder = fn(x) {
         fn(y) { x + y };
      };
      let addTwo = newAdder(2);
      addTwo(2); 
      ", 4).test_me();
}

#[test]
fn test_string_literal() {
   let input: String = String::from("\"Hello World!\"");
   match test_eval(input) {
      Some(eval) => {
         if let Some(str_obj) = eval.as_any().downcast_ref::<MkyString>() {
            assert_eq!(str_obj.value, "Hello World!")
         } else {
            panic!("object returned from test_eval is not a Monkey String")
         }
      }
      None => panic!("test_eval returned None")
   }
}