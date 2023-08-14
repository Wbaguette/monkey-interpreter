#![allow(unused)]
#[cfg(test)]

use crate::parser::ast::Program;
use crate::parser::ast::{Statement, LetStatement, Node, ReturnStatement, ExpressionStatement, Identifier, IntegerLiteral, Expression, PrefixExpression, InfixExpression, Boolean, IfExpression, FunctionLiteral, CallExpression, BlockStatement, StringLiteral, ArrayLiteral, IndexExpression};
use crate::lexer::Lexer;
use crate::parser::Parser;

use color_eyre::owo_colors::OwoColorize;
use std::any::Any;




// Some generic structs that made life easier when writing these tests
// See TestType trait in helper module to see what types are accepted



struct PrefixTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone
{
   pub input: String,
   pub operator: String,
   pub value: T,
}
impl<T> PrefixTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone
{
   pub fn new(input: &str, operator: &str, value: T) -> Self {
      PrefixTest { input: input.to_string(), operator: operator.to_string(), value }
   }

   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;
      
      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      }; 
      check_parser_errors(&parser);
      assert_eq!(program.statements.len(), 1);

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
         if let Some(prefix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<PrefixExpression>() {
            assert_eq!(prefix_expr.operator, self.operator);
            
            if self.value.is_bool() {
               test_bool_literal(prefix_expr.right.as_ref().unwrap(), self.value.downcast_bool().unwrap())
            } else if self.value.is_i64() {
               test_integer_literal(prefix_expr.right.as_ref().unwrap(), self.value.downcast_i64().unwrap())
            } else {
               panic!("{}", "Not yet implemented: 
                  !foobar => Operator: '1', value: Identifier 'foobar'
               ".bold().bright_yellow())
            }
         } else {
            panic!("expr_stmt.expression is not a PrefixExpression")
         }
      } else {
         panic!("statement is not an ExpressionStatement")
      }
   }
}



struct InfixTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   input: String,
   left_value: T,
   operator: String,
   right_value: T,
}
impl<T> InfixTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   pub fn new(input: &str, lv: T, operator: &str, rv: T) -> Self {
      InfixTest { input: input.to_string(), left_value: lv, operator: operator.to_string(), right_value: rv }
   }

   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;
      
      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      }; 
      check_parser_errors(&parser);
      assert_eq!(program.statements.len(), 1);
      
      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
         test_infix_expression(expr_stmt.expression.as_ref().unwrap(), self.left_value.clone(), self.operator.clone(), self.right_value.clone())
      } else {
         panic!("statement is not an Expression Statement")
      }
   }
}



struct Test {
   input: String, 
   expected: String,
}
impl Test {
   pub fn new(input: &str, expected: &str) -> Self {
      Test { input: input.to_string(), expected: expected.to_string() }
   }

   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;

      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      }; 
      check_parser_errors(&parser);
      assert_eq!(program.string(), self.expected)
   }
}



struct BoolTest {
   input: String,
   expected_bool: bool,
}
impl BoolTest {
   pub fn new(input: &str, b: bool) -> Self {
      BoolTest { input: input.to_string(), expected_bool: b }
   }

   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;
      
      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      }; 
      check_parser_errors(&parser);
      assert_eq!(program.statements.len(), 1);
      
      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
         test_bool_literal(expr_stmt.expression.as_ref().unwrap(), self.expected_bool)
      } else {
         panic!("statement is not an ExpressionStatement")
      }
   }
}



struct FnParamTest {
   input: String,
   expected_params: Vec<String>,
}
impl FnParamTest {
   pub fn new(input: &str, expected_params: Vec<&str>) -> Self {
      let vec: Vec<String> = expected_params.iter().map(|&s| String::from(s)).collect();
      FnParamTest { input: input.to_string(), expected_params: vec }
   }

   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;

      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      };
      check_parser_errors(&parser);

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
         if let Some(function) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<FunctionLiteral>() {
            assert_eq!(function.params.as_ref().unwrap().len(), self.expected_params.len());
         
            for (i, identifier) in self.expected_params.iter().enumerate() {
               let p: Box<dyn Expression> = Box::new(function.params.as_ref().unwrap().get(i).unwrap().clone());
               test_literal_expression(&p, identifier.to_string())
            }
         } else {
            panic!("expr_stmt.expression is not a FunctionLiteral")
         }
      } else {
         panic!("statement is not an ExpressionStatement")
      }
   }
}



struct LetStatementTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   input: String,
   expected_identifier: String,
   expected_value: T,
}
impl<T> LetStatementTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   pub fn new(input: &str, expected_identifier: &str, expected_value: T) -> Self {
      LetStatementTest { input: input.to_string(), expected_identifier: expected_identifier.to_string(), expected_value }
   }
   
   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;

      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      };
      check_parser_errors(&parser);
      assert_eq!(program.statements.len(), 1);

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      test_let_statement(statement, &self.expected_identifier);

      if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
         test_literal_expression(&let_stmt.value.as_ref().unwrap(), self.expected_value.clone())
      } else {
         panic!("statement is not a LetStatement")
      }
   }
}



struct ReturnStatementTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   input: String,
   expected_value: T,
}
impl<T> ReturnStatementTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType + Clone, 
{
   pub fn new(input: &str, expected_value: T) -> Self {
      ReturnStatementTest { input: input.to_string(), expected_value }
   }
   
   pub fn test_me(&mut self) {
      use crate::parser::ast::Program;

      let mut lexer: Lexer = Lexer::new(self.input.clone());
      let mut parser: Parser = Parser::new(lexer);
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      };
      check_parser_errors(&parser);
      assert_eq!(program.statements.len(), 1);

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
         assert_eq!(return_stmt.token_literal(), "return");
         test_literal_expression(&return_stmt.return_value.as_ref().unwrap(), self.expected_value.clone())
      } else {
         panic!("statement is not a ReturnStatement")
      }
   }
}




// GENERIC HELPER FUNCTIONS THAT MAKE THIS LIKE HUNDREDS OF LINES LESS




fn test_let_statement(statement: &Box<dyn Statement>, name: &str) {
   assert_eq!(statement.token_literal(), "let");
   
   if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
      assert_eq!(let_stmt.name.value, name.to_string());
      assert_eq!(let_stmt.name.token_literal(), name)
   } else {
      panic!("statement is a not a LetStatement. @ fn test_let_statement")
   }
}

fn check_parser_errors(parser: &Parser) {
   let errors: &Vec<String> = parser.errors();

   if errors.len() == 0 {
      return;
   }

   let mut error_msg: String = String::from(format!("Parser has {} error(s). ", errors.len()));
   for error in errors {
      error_msg.push_str(format!("\nParser has error: {}", error).as_str())
   }
   // INTENTIONAL EXPLICIT PANIC 
   panic!("{}", error_msg)
}

fn test_integer_literal(integer_literal: &Box<dyn Expression>, value: i64) {
   if let Some(int_literal) = integer_literal.as_ref().as_any().downcast_ref::<IntegerLiteral>() {
      assert_eq!(int_literal.value, value);
      assert_eq!(int_literal.token_literal(), format!("{}", value))
   } else {
      panic!("integer_literal is not an IntegerLiteral. @ fn test_integer_literal")
   }
}

fn test_identifier(expr: &Box<dyn Expression>, val: String) {
   if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
      assert_eq!(identifier.value, val);
      assert_eq!(identifier.token_literal(), val)
   } else {
      panic!("expr is not an Identifier. @ fn test_identifier")
   }
}

fn test_literal_expression<T>(expr: &Box<dyn Expression>, expected: T) 
where 
   T: std::fmt::Debug + Any + crate::helper::TestType,
{
   if expected.is_i64() {
      let int_val: i64 = expected.downcast_i64().unwrap();
      test_integer_literal(expr, int_val)
   } else if expected.is_bool() {
      let bool_val: bool = expected.downcast_bool().unwrap();
      test_bool_literal(expr, bool_val)
   } else if expected.is_string() {
      let str_val: String = expected.downcast_string().unwrap();
      test_identifier(expr, str_val)
   } else {
      panic!("Type of expr not handled. Got {:?}. @ fn test_literal_expression", expr)
   }
}

fn test_infix_expression<T>(expr: &Box<dyn Expression>, left: T, op: String, right: T) 
where 
   T: std::fmt::Debug + Any + crate::helper::TestType,
{
   if let Some(op_exp) = expr.as_any().downcast_ref::<InfixExpression>() {
      test_literal_expression(op_exp.left.as_ref().unwrap(), left);
      assert_eq!(op_exp.operator, op);
      test_literal_expression(op_exp.right.as_ref().unwrap(), right)
   } else {
      panic!("expr is not InfixExpression @ fn test_infix_expression")
   }
}

fn test_bool_literal(expr: &Box<dyn Expression>, value: bool) {
   if let Some(bool_expr) = expr.as_any().downcast_ref::<Boolean>() {
      assert_eq!(bool_expr.value, value);
      assert_eq!(bool_expr.token_literal(), format!("{}", value))
   } else {
      panic!("expr is not Boolean @ fn test_bool_literal")
   }
}











// ACTUAL TESTS

#[test]
fn test_let_statements() {
   LetStatementTest::new("let x = 5;", "x", 5).test_me();
   LetStatementTest::new("let y = true;", "y", true).test_me();
   LetStatementTest::new("let foobar = y;", "foobar", "y".to_string()).test_me();
}

#[test]
fn test_return_statements() {
   ReturnStatementTest::new("return 5;", 5).test_me();
   ReturnStatementTest::new("return 10;", 10).test_me();
   ReturnStatementTest::new("return 993322;", 993322).test_me();

   ReturnStatementTest::new("return true;", true).test_me();
   ReturnStatementTest::new("return false;", false).test_me();

   ReturnStatementTest::new("return foobar;", "foobar".to_string()).test_me();
}

#[test]
fn test_identifier_expression() {
   let input: String = String::from("foobar;");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);
   
   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      test_identifier(expr_stmt.expression.as_ref().unwrap(), String::from("foobar"));   
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_integer_literal_expression() {
   let input: String = String::from("5;");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);

   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      test_integer_literal(expr_stmt.expression.as_ref().unwrap(), 5);   
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_parsing_prefix_expressions() {
   PrefixTest::new("!5", "!", 5).test_me();
   PrefixTest::new("-15", "-", 15).test_me();

   PrefixTest::new("!true", "!", true).test_me();
   PrefixTest::new("!false", "!", false).test_me();

   // This intentionally panics if uncommented, 
   // PrefixTest::new("!foobar", "!", "foobar".to_string()).test_me();
}

#[test]
fn test_parsing_infix_expressions() {
   InfixTest::new("5 + 5", 5, "+", 5).test_me();
   InfixTest::new("5 - 5", 5, "-", 5).test_me();
   InfixTest::new("5 * 5", 5, "*", 5).test_me();
   InfixTest::new("5 / 5", 5, "/", 5).test_me();
   InfixTest::new("5 > 5", 5, ">", 5).test_me();
   InfixTest::new("5 < 5", 5, "<", 5).test_me();
   InfixTest::new("5 == 5", 5, "==", 5).test_me();
   InfixTest::new("5 != 5", 5, "!=", 5).test_me();
   
   InfixTest::new("true == true", true, "==", true).test_me();
   InfixTest::new("true != false", true, "!=", false).test_me();
   InfixTest::new("false == false", false, "==", false).test_me();
}  

#[test]
fn test_operator_precedence_parsing() {
   Test::new("-a * b", "((-a) * b)").test_me();
   Test::new("!-a", "(!(-a))").test_me();
   Test::new("a + b + c", "((a + b) + c)").test_me();
   Test::new("a + b - c", "((a + b) - c)").test_me();
   Test::new("a * b * c", "((a * b) * c)").test_me();
   Test::new("a * b / c", "((a * b) / c)").test_me();
   Test::new("a + b / c", "(a + (b / c))").test_me();
   Test::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)").test_me();
   Test::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)").test_me();
   Test::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))").test_me();
   Test::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))").test_me();
   Test::new("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))").test_me();
   Test::new("true", "true").test_me();
   Test::new("false", "false").test_me();
   Test::new("3 > 5 == false", "((3 > 5) == false)").test_me();
   Test::new("3 < 5 == true", "((3 < 5) == true)").test_me();

   Test::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)").test_me();
   Test::new("(5 + 5) * 2", "((5 + 5) * 2)").test_me();
   Test::new("2 / (5 + 5)", "(2 / (5 + 5))").test_me();
   Test::new("-(5 + 5)", "(-(5 + 5))").test_me();
   Test::new("!(true == true)", "(!(true == true))").test_me();

   Test::new("a + add(b * c) + d", "((a + add((b * c))) + d)").test_me();
   Test::new("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))").test_me();
   Test::new("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))").test_me();

   Test::new("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)").test_me();
   Test::new("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))").test_me();
}

#[test]
fn test_boolean_expression() {
   BoolTest::new("false;", false).test_me();
   BoolTest::new("true;", true).test_me();
}

#[test]
fn test_if_expression() {
   let input: String = String::from("if (x < y) { x }");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);

   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(if_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IfExpression>() {
         test_infix_expression(if_expr.condition.as_ref().unwrap(), "x".to_string(), "<".to_string(), "y".to_string());
         assert_eq!(if_expr.consequence.as_ref().unwrap().statements.len(), 1);
         if let Some(consequence) = if_expr.consequence.as_ref().unwrap().statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
            test_identifier(consequence.expression.as_ref().unwrap(), "x".to_string())
         } else {
            panic!("consequence statements[0] is not ExpressionStatement")
         }
         if if_expr.alternative.is_some() {
            panic!("if_expr.alternative is not None, got {:?}", if_expr.alternative.as_ref().unwrap())
         }
      } else {
         panic!("expression statement is not an IfExpression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IfExpression>())
      }
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_if_else_expression() {
   let input: String = String::from("if (x < y) { x } else { y }");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);
   
   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(if_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IfExpression>() {
         test_infix_expression(if_expr.condition.as_ref().unwrap(), "x".to_string(), "<".to_string(), "y".to_string());
      
         assert_eq!(if_expr.consequence.as_ref().unwrap().statements.len(), 1);   
         if let Some(consequence) = if_expr.consequence.as_ref().unwrap().statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
            test_identifier(consequence.expression.as_ref().unwrap(), "x".to_string())
         } else {
            panic!("consequence statements[0] is not ExpressionStatement")
         }

         assert_eq!(if_expr.alternative.as_ref().unwrap().statements.len(), 1);
         if let Some(alternative) = if_expr.alternative.as_ref().unwrap().statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
            test_identifier(alternative.expression.as_ref().unwrap(), "y".to_string())
         } else {
            panic!("alternative statements[0] is not ExpressionStatement")
         }
      } else {
         panic!("expression statement is not an IfExpression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IfExpression>())
      }
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_function_literal_parsing() {
   let input: String = String::from("fn(x, y) { x + y; }");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);
   
   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(function) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<FunctionLiteral>() {
         assert_eq!(function.params.as_ref().unwrap().len(), 2);
      
         let p1: Box<dyn Expression> = Box::new(function.params.as_ref().unwrap().get(0).unwrap().clone());
         let p2: Box<dyn Expression> = Box::new(function.params.as_ref().unwrap().get(1).unwrap().clone());

         test_literal_expression(&p1, "x".to_string());
         test_literal_expression(&p2, "y".to_string());

         assert_eq!(function.body.as_ref().unwrap().statements.len(), 1);
         if let Some(body_statement) = function.body.as_ref().unwrap().statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
            test_infix_expression(body_statement.expression.as_ref().unwrap(), "x".to_string(), "+".to_string() , "y".to_string())
         } else {
            panic!("function body statement is not an ExpressionStatement")
         }
      } else {
         panic!("expr.stmt is not a FunctionLiteral")
      }
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_function_parameter_parsing() {
   FnParamTest::new("fn() {};", vec![]).test_me();
   FnParamTest::new("fn(x) {};", vec!["x"]).test_me();
   FnParamTest::new("fn(x, y, z) {};", vec!["x", "y", "z"]).test_me();
}

#[test]
fn test_call_expression_parsing() {
   let input: String = String::from("add(1, 2 * 3, 4 + 5);");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser);
   assert_eq!(program.statements.len(), 1);
   
   if let Some(expr_stmt) = program.statements.get(0).as_ref().unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(call_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<CallExpression>() {
         test_identifier(&call_expr.function.as_ref().unwrap(), "add".to_string());
         assert_eq!(call_expr.arguments.as_ref().unwrap().len(), 3);
         test_literal_expression(call_expr.arguments.as_ref().unwrap().get(0).unwrap(), 1);
         test_infix_expression(call_expr.arguments.as_ref().unwrap().get(1).unwrap(), 2, "*".to_string(), 3);
         test_infix_expression(call_expr.arguments.as_ref().unwrap().get(2).unwrap(), 4, "+".to_string(), 5);
      } else {
         panic!("expr_stmt.expression is not a CallExpression")
      }
   } else {
      panic!("program.statements.get(0) is not an ExpressionStatement.")
   }
}

#[test]
fn test_string_literal_expression() {
   let input: String = String::from("\"hello world\";");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser); 

   if let Some(stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(literal) = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<StringLiteral>() {
         assert_eq!(literal.value, "hello world")
      } else {
         panic!("exp is not StringLiteral")
      }
   }
}

#[test]
fn test_parsing_array_literals() {
   let input: String = String::from("[1, 2 * 2, 3 + 3]");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser); 

   if let Some(stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(array) = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ArrayLiteral>() {
         assert_eq!(array.elements.len(), 3);
         test_integer_literal(array.elements.get(0).unwrap(), 1);
         test_infix_expression(array.elements.get(1).unwrap(), 2, "*".to_string(), 2);
         test_infix_expression(array.elements.get(2).unwrap(), 3, "+".to_string(), 3);
      } else {
         panic!("exp is not ArrayLiteral")
      }
   }
}

#[test]
fn test_parsing_index_expressions() {
   let input: String = String::from("myArray[1 + 1]");

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 
   check_parser_errors(&parser); 

   if let Some(stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(index_expr) = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IndexExpression>() {
         test_identifier(&index_expr.left, "myArray".to_string());
         test_infix_expression(&index_expr.index, 1, "+".to_string(), 1)
      } else {
         panic!("exp is not IndexExpression")
      }
   }
}
