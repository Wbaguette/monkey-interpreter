#![allow(unused)]
#[cfg(test)]

use crate::parser::ast::Program;
use crate::parser::ast::{Statement, LetStatement, Node, ReturnStatement, ExpressionStatement, Identifier, IntegerLiteral, Expression, PrefixExpression, InfixExpression, Boolean};
use crate::lexer::Lexer;
use crate::parser::Parser;

use std::any::Any;


struct IDtest {
   expected_identifier: String,
}
impl IDtest {
   pub fn new(expected_identifier: &str) -> Self {
      IDtest { expected_identifier: expected_identifier.into() }
   }
}

struct PrefixTest {
   pub input: String,
   pub operator: String,
   pub int_value: i64,
}
impl PrefixTest {
   pub fn new(input: &str, operator: &str, int_value: i64) -> Self {
      PrefixTest { input: input.to_string(), operator: operator.to_string(), int_value }
   }
}

struct PrefixTestBool {
   pub input: String,
   pub operator: String,
   pub value: bool,
}
impl PrefixTestBool {
   pub fn new(input: &str, operator: &str, value: bool) -> Self {
      PrefixTestBool { input: input.to_string(), operator: operator.to_string(), value }
   }
}   


struct InfixTest {
   input: String,
   left_value: i64,
   operator: String,
   right_value: i64,
}
impl InfixTest {
   pub fn new(input: &str, lv: i64, operator: &str, rv: i64) -> Self {
      InfixTest { input: input.to_string(), left_value: lv, operator: operator.to_string(), right_value: rv }
   }
}

struct InfixTestBool {
   input: String,
   left_value: bool,
   operator: String,
   right_value: bool,
}
impl InfixTestBool {
   pub fn new(input: &str, lv: bool, operator: &str, rv: bool) -> Self {
      InfixTestBool { input: input.to_string(), left_value: lv, operator: operator.to_string(), right_value: rv }
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
}

struct BoolTest {
   input: String,
   expected_bool: bool,
   expected_lit: String,
}
impl BoolTest {
   pub fn new(input: &str, b: bool, lit: &str) -> Self {
      BoolTest { input: input.to_string(), expected_bool: b, expected_lit: lit.to_string() }
   }
}


fn test_let_statement(statement: &Box<dyn Statement>, name: &str) {
   if statement.token_literal() != "let" {
      panic!("statement.token_literal() is {}. Expected 'let'", statement.token_literal());
   }

   if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
      
      if let_stmt.name.value != name.to_string() {
         panic!("LetStatement.name.value is {}. Expected: {}", let_stmt.name.value, name)
      }
      if let_stmt.name.token_literal() != name {
         panic!("LetStatement.name.token_literal() is {}. Expected: {}", let_stmt.name.token_literal(), name)
      }

   } else {
      panic!("statement is a not a LetStatement.")
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



// I probably will never use these helper test functions

fn test_integer_literal(integer_literal: &Box<dyn Expression>, value: i64) {
   if let Some(int_literal) = integer_literal.as_ref().as_any().downcast_ref::<IntegerLiteral>() {
      if int_literal.value != value {
         panic!("int_literal.value is {}. Expected: {}", int_literal.value, value)
      }
      if int_literal.token_literal() != format!("{}", value) {
         panic!("int_literal.token_literal() is {}. Expected: {}", int_literal.token_literal(), value)
      }
   } else {
      panic!("integer_literal is not an IntegerLiteral")
   }
}

fn test_identifier(expr: &Box<dyn Expression>, val: String) {
   if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
      if identifier.value != val {
         panic!("identifier.value is {}. Expected {}", identifier.value, val)
      }
      if identifier.token_literal() != val {
         panic!("identifier.token_literal() is {}. Expected {}", identifier.token_literal(), val)
      }
   } else {
      panic!("expression is not an Identifier")
   }
}

fn test_literal_expression(expr: &Box<dyn Expression>, expected: &dyn Any) {
   if let Some(int) = expected.downcast_ref::<i64>() {
      test_integer_literal(expr, int.to_owned())
   } else if let Some(string) = expected.downcast_ref::<String>() {
      test_identifier(expr, string.to_owned())
   } else if let Some(boolean) = expected.downcast_ref::<Boolean>() {
      test_bool_literal(expr, boolean)
   }
   panic!("type of exp not handled")
}

fn test_infix_expression(expr: &Box<dyn Expression>, left: &dyn Any, op: String, right: &dyn Any) {
   if let Some(op_exp) = expr.as_any().downcast_ref::<InfixExpression>() {
      test_literal_expression(op_exp.left.as_ref().unwrap(), left);
      if op_exp.operator != op {
         panic!("op_exp.operator is {}. Expected: {}", op_exp.operator, op)
      }
   } else {
      panic!("expr is not InfixExpression @ fn test_infix_expression")
   }
}

fn test_bool_literal(expr: &Box<dyn Expression>, expected: &dyn Any) {

}







#[test]
fn test_let_statements() {
   
   let input: String = String::from(
   "  let x = 5;
      let y = 10;
      let foobar = 838383;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);

   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 

   check_parser_errors(&parser);

   if program.statements.len() != 3 {
      panic!("program.statements contains {} statements. Expected 3 statements", program.statements.len())
   }

   let tests: Vec<IDtest> = vec![
      IDtest::new("x"),
      IDtest::new("y"),
      IDtest::new("foobar"),
   ];

   for (idx, test) in tests.iter().enumerate() {
      let statement: &Box<dyn Statement> = program.statements.get(idx).unwrap();
      test_let_statement(statement, &test.expected_identifier)
   }

}

#[test]
fn test_return_statements() {
   
   let input: String = String::from(
   "  return 5;
      return 10;
      return 993322;
   "
   );

   let mut lexer: Lexer = Lexer::new(input);
   let mut parser: Parser = Parser::new(lexer);

   let program: Program = match parser.parse_program() {
      Ok(program) => program,
      Err(e) => panic!("{}", e),
   }; 

   check_parser_errors(&parser);

   if program.statements.len() != 3 {
      panic!("program.statements contains {} statements. Expected 3 statements", program.statements.len())
   }

   for statement in program.statements {
      if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
      
         if return_stmt.token_literal() != "return" {
            panic!("ReturnStatement.token_literal() is {}. Expected: 'return'", return_stmt.token_literal())
         }
         
      } else {
         panic!("statement is a not a ReturnStatement.")
      }
   }
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
      if let Some(identifier) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<Identifier>() {
         if identifier.value != "foobar" {
            panic!("identifier.value is {}. Expected: 'foobar'", identifier.value)
         }
         if identifier.token_literal() != "foobar" {
            panic!("identifier.token_literal() is {}. Expected 'foobar'", identifier.token_literal())
         }
      } else {
         panic!("expression statement is not an Identifier. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<Identifier>())
      }
      
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
      if let Some(int_literal) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<IntegerLiteral>() {
         if int_literal.value != 5 {
            panic!("identifier.value is {}. Expected: 5", int_literal.value)
         }
         if int_literal.token_literal() != "5" {
            panic!("identifier.token_literal() is {}. Expected '5'", int_literal.token_literal())
         }
      } else {
         panic!("expression statement is not an IntegerLiteral. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
      }
      
   } else {
      panic!("program.statements.get(0) is a not an ExpressionStatement.")
   }
}

#[test]
fn test_parsing_prefix_expressions_i64() {
   let tests: Vec<PrefixTest> = vec![
      PrefixTest::new("!5", "!", 5),
      PrefixTest::new("-15", "-", 15),
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
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
         if let Some(prefix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<PrefixExpression>() {
            if prefix_expr.operator != test.operator {
               panic!("prefix_expr.operator is {}. Expected: {}", prefix_expr.operator, test.operator)
            }

            test_integer_literal(prefix_expr.right.as_ref().unwrap(), test.int_value);
         } else {
            panic!("expression statement is not a prefix expression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
         }
         
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }

   }
}

#[test]
fn test_parsing_prefix_expressions_bool() {
   let tests: Vec<PrefixTestBool> = vec![
      PrefixTestBool::new("!true", "!", true),
      PrefixTestBool::new("!false", "!", false),
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
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
         if let Some(prefix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<PrefixExpression>() {
            if let Some(boolean) = prefix_expr.right.as_ref().unwrap().as_any().downcast_ref::<Boolean>() {
               if boolean.value != test.value {
                  panic!("boolean.value is {}. Expected: {}", boolean.value, test.value)
               }
            } 
            if prefix_expr.operator != test.operator {
               panic!("prefix_expr.operator is {}. Expected: {}", prefix_expr.operator, test.operator)
            }
         } else {
            panic!("expression statement is not a prefix expression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
         }
         
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }

   }
}

#[test]
fn test_parsing_infix_expressions_i64() {
   let tests: Vec<InfixTest> = vec![
      InfixTest::new("5 + 5", 5, "+", 5),
      InfixTest::new("5 - 5", 5, "-", 5),
      InfixTest::new("5 * 5", 5, "*", 5),
      InfixTest::new("5 / 5", 5, "/", 5),
      InfixTest::new("5 > 5", 5, ">", 5),
      InfixTest::new("5 < 5", 5, "<", 5),
      InfixTest::new("5 == 5", 5, "==", 5),
      InfixTest::new("5 != 5", 5, "!=", 5),
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
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
         if let Some(infix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<InfixExpression>() {
            test_integer_literal(infix_expr.left.as_ref().unwrap(), test.left_value);
            if infix_expr.operator != test.operator {
               panic!("infix_expr.operator is {}. Expected {}", infix_expr.operator, test.operator)
            }
            test_integer_literal(infix_expr.right.as_ref().unwrap(), test.right_value);
         } else {
            panic!("expression statement is not an infix expression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
         }
         
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }
   }
}  

#[test]
fn test_parsing_infix_expressions_bool() {
   let tests: Vec<InfixTestBool> = vec![
      InfixTestBool::new("true == true", true, "==", true),
      InfixTestBool::new("true != false", true, "!=", false),
      InfixTestBool::new("false == false", false, "==", false),
      
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
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
         if let Some(infix_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<InfixExpression>() {
            // Have an infixexpression, need to check if the left side is a boolean, and if it is then test if it equals test.left...
            if let Some(left_bool) = infix_expr.left.as_ref().unwrap().as_any().downcast_ref::<Boolean>() {
               if left_bool.value != test.left_value {
                  panic!("left_bool.value is {}. Expected: {}", left_bool.value, test.left_value)
               }
            } else {
               panic!("infix_expr.left is not a boolean: {:?}", infix_expr.left.as_ref().unwrap().as_any().downcast_ref::<bool>())
            }
            // Test operator
            if infix_expr.operator != test.operator {
               panic!("infix_expr.operator is {}. Expected {}", infix_expr.operator, test.operator)
            }
            // Have an infixexpression, need to check if the right side is a boolean, and if it is then test if it equals test.right...
            if let Some(right_bool) = infix_expr.right.as_ref().unwrap().as_any().downcast_ref::<Boolean>() {
               if right_bool.value != test.right_value {
                  panic!("left_bool.value is {}. Expected: {}", right_bool.value, test.right_value)
               }
            } else {
               panic!("infix_expr.left is not a boolean: {:?}", infix_expr.left.as_ref().unwrap().as_any().downcast_ref::<bool>())
            }

         } else {
            panic!("expression statement is not an infix expression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
         }
         
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }
      

      
   }
}  

#[test]
fn test_operator_precedence_parsing() {
   let tests: Vec<Test> = vec![
      Test::new("-a * b", "((-a) * b)"),
      Test::new("!-a", "(!(-a))"),
      Test::new("a + b + c", "((a + b) + c)"),
      Test::new("a + b - c", "((a + b) - c)"),
      Test::new("a * b * c", "((a * b) * c)"),
      Test::new("a * b / c", "((a * b) / c)"),
      Test::new("a + b / c", "(a + (b / c))"),
      Test::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
      Test::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
      Test::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
      Test::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
      Test::new("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
      Test::new("true", "true"),
      Test::new("false", "false"),
      Test::new("3 > 5 == false", "((3 > 5) == false)"),
      Test::new("3 < 5 == true", "((3 < 5) == true)"),

      Test::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
      Test::new("(5 + 5) * 2", "((5 + 5) * 2)"),
      Test::new("2 / (5 + 5)", "(2 / (5 + 5))"),
      Test::new("-(5 + 5)", "(-(5 + 5))"),
      Test::new("!(true == true)", "(!(true == true))"),
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
      let mut parser: Parser = Parser::new(lexer);
   
      let program: Program = match parser.parse_program() {
         Ok(program) => program,
         Err(e) => panic!("{}", e),
      }; 
   
      check_parser_errors(&parser);

      let actual: String = program.string();
      if actual != test.expected {
         panic!("Program string representation is {}. Expected: {}", actual, test.expected);
      }
   }
}

#[test]
fn test_boolean_expression() {
   let tests: Vec<BoolTest> = vec![
      BoolTest::new("false;", false, "false"),
      BoolTest::new("true;", true, "true"),
   ];

   for test in tests {
      let mut lexer: Lexer = Lexer::new(test.input);
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
         if let Some(boolean) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<Boolean>() {
            if boolean.value != test.expected_bool {
               panic!("boolean.value is {}. Expected: {}", boolean.value, test.expected_bool)
            }
            if boolean.token_literal() != test.expected_lit {
               panic!("boolean.token_literal() is {}. Expected {}", boolean.token_literal(), test.expected_lit)
            }
         } else {
            panic!("expression statement is not a Boolean. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<Boolean>())
         }
         
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }
   }
}
