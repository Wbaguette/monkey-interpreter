#![allow(unused)]

use crate::objects::Integer;
use crate::{objects::Object, parser::Parser};
use crate::lexer::Lexer;
use crate::parser::ast::{Program, Node};
use crate::evaluator::eval;

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
      let evaluated: Box<dyn Object> = match test_eval(self.input.clone()) {
         Some(obj) => obj,
         None => panic!("Evaluated returned None"),
      };

      test_integer_object(evaluated, self.expected)
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




// ACTUAL TEST FUNCTIONS




#[test]
fn test_eval_integer_expression() {
   i64Test::new("5", 5).test_me();
   i64Test::new("10", 10).test_me();
}