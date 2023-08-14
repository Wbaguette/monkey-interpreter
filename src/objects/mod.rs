pub mod environment;

use std::any::Any;
use dyn_clone::DynClone;
use crate::parser::ast::{Identifier, BlockStatement, Node};
use self::environment::Environment;

#[derive(Debug, Clone)]
pub enum ObjectTypes {
   IntegerObj,
   BooleanObj,
   NullObj,
   ReturnValObj,
   ErrorObj,
   FunctionObj,
   StringObj,
}
impl ObjectTypes {
   pub fn to_string(&self) -> String {
      return match self {
         Self::IntegerObj => "INTEGER",
         Self::BooleanObj => "BOOLEAN",
         Self::NullObj => "NULL",
         Self::ReturnValObj => "RETURN_VALUE",
         Self::ErrorObj => "ERROR",
         Self::FunctionObj => "FUNCTION",
         Self::StringObj => "STRING",
      }.to_string()
   }
}

type ObjectType = String;
pub trait Object: DynClone {
   fn r#type(&self) -> ObjectType;
   fn inspect(&self) -> String;
   fn as_any(&self) -> &dyn Any;
}
impl std::fmt::Debug for dyn Object {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(self.inspect().as_str())
   }
}
dyn_clone::clone_trait_object!(Object);









#[derive(Copy, Clone, Debug, PartialEq)]         
pub struct Integer {
   pub value: i64,
}
impl Object for Integer {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::IntegerObj.to_string()
   }

   fn inspect(&self) -> String {
      format!("{}", self.value)
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

#[derive(Copy, Clone, Debug, PartialEq)] 
pub struct Boolean {
   pub value: bool,
}
impl Object for Boolean {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::BooleanObj.to_string()   
   }

   fn inspect(&self) -> String {
      format!("{}", self.value)
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

#[derive(Copy, Clone, Debug, PartialEq)] 
pub struct Null;
impl Object for Null {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::NullObj.to_string()
   }

   fn inspect(&self) -> String {
      String::from("null")
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

#[derive(Clone, Debug)]
pub struct ReturnValue {
   pub value: Box<dyn Object>,
}
impl Object for ReturnValue {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::ReturnValObj.to_string()
   }

   fn inspect(&self) -> String {
      self.value.inspect()
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

#[derive(Clone, Debug, PartialEq)] 
pub struct Error {
   pub message: String,
}
impl Error {
   pub fn new(message: String) -> Self {
      Error { message }
   }
}
impl Object for Error {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::ErrorObj.to_string()
   }

   fn inspect(&self) -> String {
      String::from(format!("ERROR: {}", self.message))
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

#[derive(Clone, Debug)] 
pub struct Function {
   pub params: Option<Vec<Identifier>>,      // Situation: No Params in function => params becomes Some(empty vec), 
                                             // Situation: Params fuck up/syntax is wrong => params is None
   pub body: Option<BlockStatement>,
   // This environment is a copy of the overall environment that a function is CALLED from
   pub env: Environment,
}
impl Object for Function {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::FunctionObj.to_string()
   }

   fn inspect(&self) -> String {
      let mut out: String = String::new();
      let mut params: Vec<String> = vec![];
      for p in self.params.as_ref().unwrap() {
         params.push(p.string())
      }

      out.push_str("fn (");
      out.push_str(params.join(", ").as_str());
      out.push_str(") {\n");
      out.push_str(self.body.as_ref().unwrap().string().as_str());
      out.push_str("\n}");

      out
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}

// MkyString => "Monkey String" since using "String" as a name is no bueno
#[derive(Clone, Debug)] 
pub struct MkyString {
   pub value: String,
}
impl Object for MkyString {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::StringObj.to_string()
   }

   fn inspect(&self) -> String {
      self.value.clone()
   }

   fn as_any(&self) -> &dyn Any {
      self
   }
}