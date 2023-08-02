#![allow(unused)]
#[cfg(test)]

use crate::parser::ast::Program;
use crate::parser::ast::{Statement, LetStatement, Node, ReturnStatement, ExpressionStatement, Identifier, IntegerLiteral, Expression, PrefixExpression, InfixExpression, Boolean, IfExpression, FunctionLiteral, CallExpression};
use crate::lexer::Lexer;
use crate::parser::Parser;

use color_eyre::owo_colors::OwoColorize;
use std::any::Any;




// SOME GENERIC STRUCTS FOR HELPER FUNCTIONS AND MORE GENERIC TEST CASES
// THIS SHIT REMOVED LIKE 100 LINES OF REPEATED CODE...




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
   
      if program.statements.len() != 1 {
         panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
      }

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
         if let Some(prefix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<PrefixExpression>() {
            if prefix_expr.operator != self.operator {
               panic!("prefix_expr.operator is {}. Expected: {}", prefix_expr.operator, self.operator)
            }
            
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
   
      if program.statements.len() != 1 {
         panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
      }

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

      let actual: String = program.string();
      if actual != self.expected {
         panic!("Program string representation is {}. Expected: {}", actual, self.expected);
      } 
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
   
      if program.statements.len() != 1 {
         panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
      }

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
            if function.params.as_ref().unwrap().len() != self.expected_params.len() {
               panic!("Length of parameters is wrong: Expected {}. Got {}",
                  self.expected_params.len(), function.params.as_ref().unwrap().len())
            }
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

      if program.statements.len() != 1 {
         panic!("program.statements contains {} statements. Expected: 1", program.statements.len())
      }

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

      if program.statements.len() != 1 {
         panic!("program.statements contains {} statements. Expected: 1", program.statements.len())
      }

      let statement: &Box<dyn Statement> = program.statements.get(0).unwrap();
      if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
         if return_stmt.token_literal() != "return" {
            panic!("return_stmt.token_literal() is not 'return'")
         }

         test_literal_expression(&return_stmt.return_value.as_ref().unwrap(), self.expected_value.clone())
      } else {
         panic!("statement is not a ReturnStatement")
      }
   }
}




// GENERIC HELPER FUNCTIONS THAT MAKE THIS LIKE HUNDREDS OF LINES LESS




fn test_let_statement(statement: &Box<dyn Statement>, name: &str) {
   if statement.token_literal() != "let" {
      panic!("statement.token_literal() is {}. Expected 'let'. @ fn test_let_statement", statement.token_literal());
   }

   if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
      if let_stmt.name.value != name.to_string() {
         panic!("LetStatement.name.value is {}. Expected: {}. @ fn test_let_statement", let_stmt.name.value, name)
      }
      if let_stmt.name.token_literal() != name {
         panic!("LetStatement.name.token_literal() is {}. Expected: {}. @ fn test_let_statement", let_stmt.name.token_literal(), name)
      }
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
   panic!("{}", error_msg);
}

fn test_integer_literal(integer_literal: &Box<dyn Expression>, value: i64) {
   if let Some(int_literal) = integer_literal.as_ref().as_any().downcast_ref::<IntegerLiteral>() {
      if int_literal.value != value {
         panic!("int_literal.value is {}. Expected: {}. @ fn test_integer_literal", int_literal.value, value)
      }
      if int_literal.token_literal() != format!("{}", value) {
         panic!("int_literal.token_literal() is {}. Expected: {}. @ fn test_integer_literal", int_literal.token_literal(), value)
      }
   } else {
      panic!("integer_literal is not an IntegerLiteral. @ fn test_integer_literal")
   }
}

fn test_identifier(expr: &Box<dyn Expression>, val: String) {
   if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
      if identifier.value != val {
         panic!("identifier.value is {}. Expected {}. @ fn test_identifier", identifier.value, val)
      }
      if identifier.token_literal() != val {
         panic!("identifier.token_literal() is {}. Expected {}. @ fn test_identifier", identifier.token_literal(), val)
      }
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
      if op_exp.operator != op {
         panic!("op_exp.operator is {}. Expected: {}. @ fn test_infix_expression", op_exp.operator, op)
      }
      test_literal_expression(op_exp.right.as_ref().unwrap(), right)
   } else {
      panic!("expr is not InfixExpression @ fn test_infix_expression")
   }
}

fn test_bool_literal(expr: &Box<dyn Expression>, value: bool) {
   if let Some(bool_expr) = expr.as_any().downcast_ref::<Boolean>() {
      if bool_expr.value != value {
         panic!("bool_expr.value is {}. Expected {}. @ fn test_bool_literal", bool_expr.value, value)
      }
      if bool_expr.token_literal() != format!("{value}") {
         panic!("bool_expr.token_literal() is {}. Expected {}. @ fn test_bool_literal", bool_expr.token_literal(), value)
      }
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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(if_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IfExpression>() {
         test_infix_expression(if_expr.condition.as_ref().unwrap(), "x".to_string(), "<".to_string(), "y".to_string());
         
         if if_expr.consequence.as_ref().unwrap().statements.len() != 1 {
            panic!("consequence is not 1 statement: Got {} statements.", if_expr.consequence.as_ref().unwrap().statements.len())
         }

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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(if_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IfExpression>() {
         test_infix_expression(if_expr.condition.as_ref().unwrap(), "x".to_string(), "<".to_string(), "y".to_string());
         
         if if_expr.consequence.as_ref().unwrap().statements.len() != 1 {
            panic!("consequence is not 1 statement: Got {} statements.", if_expr.consequence.as_ref().unwrap().statements.len())
         }

         if let Some(consequence) = if_expr.consequence.as_ref().unwrap().statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
            test_identifier(consequence.expression.as_ref().unwrap(), "x".to_string())
         } else {
            panic!("consequence statements[0] is not ExpressionStatement")
         }



         if if_expr.alternative.as_ref().unwrap().statements.len() != 1 {
            panic!("alternative is not 1 statement: Got {} statements.", if_expr.alternative.as_ref().unwrap().statements.len())
         }

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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

   if let Some(expr_stmt) = program.statements.get(0).unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(function) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<FunctionLiteral>() {
         if function.params.as_ref().unwrap().len() != 2 {
            panic!("function.params is {}. Expected: 2", function.params.as_ref().unwrap().len())
         }

         let p1: Box<dyn Expression> = Box::new(function.params.as_ref().unwrap().get(0).unwrap().clone());
         let p2: Box<dyn Expression> = Box::new(function.params.as_ref().unwrap().get(1).unwrap().clone());

         test_literal_expression(&p1, "x".to_string());
         test_literal_expression(&p2, "y".to_string());

         if function.body.as_ref().unwrap().statements.len() != 1 {
            panic!("function.body.statements.len() is {}. Expected: 1", function.body.as_ref().unwrap().statements.len())
         }

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

   if program.statements.len() != 1 {
      panic!("program.statements contains {} statements. Expected 1 statement", program.statements.len())
   }

   if let Some(expr_stmt) = program.statements.get(0).as_ref().unwrap().as_any().downcast_ref::<ExpressionStatement>() {
      if let Some(call_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<CallExpression>() {
         test_identifier(&call_expr.function.as_ref().unwrap(), "add".to_string());
         if call_expr.arguments.as_ref().unwrap().len() != 3 {
            panic!("Wrong length of arguments. Got {}. Expected 3", call_expr.arguments.as_ref().unwrap().len())
         }

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
