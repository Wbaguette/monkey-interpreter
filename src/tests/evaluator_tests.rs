#![allow(unused)]

use std::any::Any;

use crate::objects::{Integer, Boolean, Object, Null};
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::parser::ast::Program;
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

   pub fn test_me(&mut self) {
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

   pub fn test_me(&mut self) {
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

   pub fn test_me(&mut self) {
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




// HELPER FUNCTIONS




fn test_eval(input: String) -> Option<Box<dyn Object>> {
   let lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(p) => p,
      Err(e) => panic!("{}", e),
   };

   return eval(Box::new(&program));
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
