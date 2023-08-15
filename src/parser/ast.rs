use dyn_clone::DynClone;
use crate::lexer::token::Token;
use std::{any::Any, collections::{HashMap, hash_map::DefaultHasher}, hash::Hasher};

pub trait Node: DynClone {
   fn token_literal(&self) -> &str;
   fn string(&self) -> String;
   fn node_as_any(&self) -> &dyn Any;
}
dyn_clone::clone_trait_object!(Node);

pub trait Statement: Node + Any + DynClone {
   fn statement_node(&self);
   fn as_any(&self) -> &dyn Any;
   fn as_node(&self) -> &dyn Node;
}
impl std::fmt::Debug for dyn Statement {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(format!("{}", self.string()).as_str())
   }
}
dyn_clone::clone_trait_object!(Statement);

pub trait Expression: Node + Any + DynClone {
   fn expression_node(&self);
   fn as_any(&self) -> &dyn Any;
   fn as_node(&self) -> &dyn Node;
}
impl std::fmt::Debug for dyn Expression {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(format!("{}", self.string()).as_str())
   }
}
dyn_clone::clone_trait_object!(Expression);

impl Eq for dyn Expression {}
impl std::hash::Hash for dyn Expression {
   fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      let mut hasher: DefaultHasher = DefaultHasher::new();
      std::ptr::hash(self, &mut hasher);
      hasher.finish().hash(state)
   }
}
impl PartialEq<dyn Expression> for dyn Expression {
   fn eq(&self, other: &dyn Expression) -> bool {
      std::ptr::eq(self, other)
   }
}




#[derive(Debug, Clone)]
pub struct Program {
   pub statements: Vec<Box<dyn Statement>>,
}
impl Node for Program {
   fn token_literal(&self) -> &str {
      if self.statements.len() > 0 {
         return self.statements.get(0).unwrap().token_literal()
      } else {
         ""
      }
   }

   fn string(&self) -> String {
      let mut out: String = String::new();

      for s in &self.statements {
         out.push_str(s.string().as_str());
      }
      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
   pub token: Token,  // this should always be TokenType::IDENT   (binding)
   pub value: String,
}
impl Node for Identifier {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.value.clone()
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for Identifier {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct LetStatement {
   pub token: Token,   // this should always be TokenType::LET
   pub name: Identifier,   // LetStatement.name.token_literal() should return the binding value: let foo = 5;   => "foo"
   pub value: Option<Box<dyn Expression>>,
}
impl Node for LetStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("{} ", self.token_literal()).as_str());
      out.push_str(format!("{} = ", self.name.string()).as_str());

      if self.value.is_some() {
         out.push_str(self.value.as_ref().unwrap().string().as_str());
      }

      out.push_str(";");

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Statement for LetStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct ReturnStatement {
   pub token: Token,
   pub return_value: Option<Box<dyn Expression>>,
}
impl Node for ReturnStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("{} ", self.token_literal()).as_str());

      if self.return_value.is_some() {
         out.push_str(self.return_value.as_ref().unwrap().string().as_str());
      }

      out.push_str(";");

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Statement for ReturnStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct ExpressionStatement {
   pub token: Token,
   pub expression: Option<Box<dyn Expression>>,
}
impl Node for ExpressionStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      if self.expression.is_some() {
         out.push_str(self.expression.as_ref().unwrap().string().as_str());
      }
      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Statement for ExpressionStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
   pub token: Token,
   pub value: i64,
}
impl Node for IntegerLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.token.literal.clone()
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for IntegerLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct PrefixExpression {
   pub token: Token,
   pub operator: String,
   pub right: Option<Box<dyn Expression>>,
}
impl Node for PrefixExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      out.push_str(format!("({}", self.operator).as_str());
      out.push_str(format!("{})", self.right.as_ref().unwrap().string()).as_str());

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for PrefixExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct InfixExpression {
   pub token: Token, 
   pub left: Option<Box<dyn Expression>>,
   pub operator: String,
   pub right: Option<Box<dyn Expression>>,
}
impl Node for InfixExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }
   
   fn string(&self) -> String {
      let mut out: String = String::new();

      out.push_str(format!("({}", self.left.as_ref().unwrap().string()).as_str());
      out.push_str(format!(" {} ", self.operator).as_str());
      out.push_str(format!("{})", self.right.as_ref().unwrap().string()).as_str());
      
      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for InfixExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
   pub token: Token,
   pub value: bool,
}
impl Node for Boolean {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.token.literal.clone()
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for Boolean {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct IfExpression {
   pub token: Token,
   pub condition: Option<Box<dyn Expression>>,
   pub consequence: Option<BlockStatement>,
   pub alternative: Option<BlockStatement>,
}
impl Node for IfExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();

      out.push_str(format!("if {} {}", self.condition.as_ref().unwrap().string(), self.consequence.as_ref().unwrap().string() ).as_str());
      if self.alternative.is_some() {
         out.push_str(format!("else {}", self.alternative.as_ref().unwrap().string()).as_str());
      }

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for IfExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct BlockStatement {
   pub token: Token,
   pub statements: Vec<Box<dyn Statement>>,
}
impl Node for BlockStatement {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();

      for s in &self.statements {
         out.push_str(s.string().as_str())
      }

      out
   } 

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Statement for BlockStatement {
   fn statement_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct FunctionLiteral {
   pub token: Token,
   pub params: Option<Vec<Identifier>>,      // Situation: No Params in function => params becomes Some(empty vec), 
                                             // Situation: Params fuck up/syntax is wrong => params is None
   pub body: Option<BlockStatement>,
}
impl Node for FunctionLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      let mut params_str: Vec<String> = vec![];

      for p in self.params.as_ref().unwrap() {
         params_str.push(p.string())
      }

      out.push_str(format!("{}(", self.token_literal()).as_str());
      out.push_str(params_str.join(", ").as_str());
      out.push_str(format!("){}", self.body.as_ref().unwrap().string()).as_str());
      
      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for FunctionLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct CallExpression {
   pub token: Token,
   pub function: Option<Box<dyn Expression>>,   // Identifier or FunctionLiteral
   pub arguments: Option<Vec<Box<dyn Expression>>>, 
}
impl Node for CallExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      let mut args_str: Vec<String> = vec![];

      for expr in self.arguments.as_ref().unwrap() {
         args_str.push(expr.string())
      }

      out.push_str(format!("{}(", self.function.as_ref().unwrap().string()).as_str());
      out.push_str(format!("{})", args_str.join(", ")).as_str());

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for CallExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
   pub token: Token,
   pub value: String,
}
impl Node for StringLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      self.token.literal.clone()
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for StringLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct ArrayLiteral {
   pub token: Token,                              // The LBRACKET token "["
   pub elements: Vec<Box<dyn Expression>>,        // Arrays can host multiple types 
}
impl Node for ArrayLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      let mut e: Vec<String> = Vec::new();

      for el in &self.elements {
         e.push(el.string())
      }
      out.push_str("[");
      out.push_str(e.join(", ").as_str());
      out.push_str("]");

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for ArrayLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct IndexExpression {
   pub token: Token,                   // The LBRACKET token "["
   pub left: Box<dyn Expression>,
   pub index: Box<dyn Expression>,
}
impl Node for IndexExpression {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();

      out.push_str("(");
      out.push_str(self.left.string().as_str());
      out.push_str("[");
      out.push_str(self.index.string().as_str());
      out.push_str("])");

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for IndexExpression {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}



#[derive(Debug, Clone)]
pub struct HashLiteral {
   pub token: Token,
   pub pairs: HashMap<Box<dyn Expression>, Box<dyn Expression>>,
}
impl Node for HashLiteral {
   fn token_literal(&self) -> &str {
      self.token.literal.as_str()
   }

   fn string(&self) -> String {
      let mut out: String = String::new();
      let mut pairs: Vec<String> = Vec::new();

      for (k, v) in &self.pairs {
         pairs.push(format!("{}:{}", k.string(), v.string()))
      }
      out.push_str("{");
      out.push_str(pairs.join(", ").as_str());
      out.push_str("}");

      out
   }

   fn node_as_any(&self) -> &dyn Any {
      self
   }
}
impl Expression for HashLiteral {
   fn expression_node(&self) {}
   fn as_any(&self) -> &dyn Any {
      self
   }
   fn as_node(&self) -> &dyn Node {
      self
   }
}