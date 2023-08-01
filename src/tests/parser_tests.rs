#![allow(unused)]
#[cfg(test)]

use crate::parser::ast::Program;
use crate::parser::ast::{Statement, LetStatement, Node, ReturnStatement, ExpressionStatement, Identifier, IntegerLiteral, Expression, PrefixExpression, InfixExpression, Boolean, IfExpression};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::parser::Parser;

use std::any::{Any, TypeId};






// SOME GENERIC STRUCTS FOR HELPER FUNCTIONS AND MORE GENERIC TEST CASES

struct IDtest {
   expected_identifier: String,
}
impl IDtest {
   pub fn new(expected_identifier: &str) -> Self {
      IDtest { expected_identifier: expected_identifier.into() }
   }
}

struct PrefixTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType 
{
   pub input: String,
   pub operator: String,
   pub value: T,
}
impl<T> PrefixTest<T>
where T: std::fmt::Debug + Any + crate::helper::TestType 
{
   pub fn new(input: &str, operator: &str, value: T) -> Self {
      PrefixTest { input: input.to_string(), operator: operator.to_string(), value }
   }
}
// JUMP: PREFIXBOOL DEF
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


struct InfixTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType, 
{
   input: String,
   left_value: T,
   operator: String,
   right_value: T,
}
impl<T> InfixTest<T> 
where T: std::fmt::Debug + Any + crate::helper::TestType, 
{
   pub fn new(input: &str, lv: T, operator: &str, rv: T) -> Self {
      InfixTest { input: input.to_string(), left_value: lv, operator: operator.to_string(), right_value: rv }
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
fn test_parsing_prefix_expressions_i64() {
   let tests: Vec<PrefixTest<i64>> = vec![
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
            test_integer_literal(prefix_expr.right.as_ref().unwrap(), test.value);
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
   let tests: Vec<PrefixTest<bool>> = vec![
      PrefixTest::new("!true", "!", true),
      PrefixTest::new("!false", "!", false),
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
            test_bool_literal(prefix_expr.right.as_ref().unwrap(), test.value);
         } else {
            panic!("expression statement is not a prefix expression. \nGot: {:?}", expr_stmt.as_any().downcast_ref::<IntegerLiteral>())
         }
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }

   }
}

#[test]
fn test_parsing_infix_expressions() {
   let tests_i64: Vec<InfixTest<i64>> = vec![
      InfixTest::new("5 + 5", 5, "+", 5),
      InfixTest::new("5 - 5", 5, "-", 5),
      InfixTest::new("5 * 5", 5, "*", 5),
      InfixTest::new("5 / 5", 5, "/", 5),
      InfixTest::new("5 > 5", 5, ">", 5),
      InfixTest::new("5 < 5", 5, "<", 5),
      InfixTest::new("5 == 5", 5, "==", 5),
      InfixTest::new("5 != 5", 5, "!=", 5),
   ];

   for test in tests_i64 {
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
         test_infix_expression(expr_stmt.expression.as_ref().unwrap(), test.left_value, test.operator, test.right_value)
      } else {
         panic!("program.statements.get(0) is a not an ExpressionStatement.")
      }
   }

   let tests_bool: Vec<InfixTest<bool>> = vec![
      InfixTest::new("true == true", true, "==", true),
      InfixTest::new("true != false", true, "!=", false),
      InfixTest::new("false == false", false, "==", false),
   ];

   for test in tests_bool {
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
         test_infix_expression(expr_stmt.expression.as_ref().unwrap(), test.left_value, test.operator, test.right_value);
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

// TODO: Implement this 
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