pub mod environment;

use std::{any::Any, collections::{hash_map::DefaultHasher, HashMap}, hash::Hasher};
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
   BuiltInObj, 
   ArrayObj,
   HashObj,
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
         Self::BuiltInObj => "BUILTIN",
         Self::ArrayObj => "ARRAY",
         Self::HashObj => "HASH",
      }.to_string()
   }
}



type ObjectType = String;
pub trait Object: DynClone {
   fn r#type(&self) -> ObjectType;
   fn inspect(&self) -> String;
   fn as_any(&self) -> &dyn Any;

   fn is_hashable(&self) -> bool;
   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>>;
}
impl std::fmt::Debug for dyn Object {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(self.inspect().as_str())
   }
}
// impl Eq for dyn Object {}
// impl PartialEq<dyn Object> for dyn Object {
//    fn eq(&self, other: &dyn Object) -> bool {
//       std::ptr::eq(self, other)
//    }
// }
// impl Ord for dyn Object {
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       
//    }
// }
// impl PartialOrd for dyn Object {

// }
dyn_clone::clone_trait_object!(Object);



#[derive(Clone, Debug, Eq, PartialEq, Hash)] 
pub struct HashKey {
   pub r#type: ObjectType,
   pub value: u64,
}
pub trait Hashable {
   fn hash_key(&self) -> HashKey;
}






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

   fn is_hashable(&self) -> bool {
      true
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      Some(Box::new(*self))
   }
}
impl Hashable for Integer {
   fn hash_key(&self) -> HashKey {
      HashKey { r#type: self.r#type(), value: self.value as u64 }
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

   fn is_hashable(&self) -> bool {
      true
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      Some(Box::new(*self))
   }
}
impl Hashable for Boolean {
   fn hash_key(&self) -> HashKey {
      let val: u64;

      if self.value {
         val = 1
      } else {
         val = 0
      }

      HashKey { r#type: self.r#type(), value: val }
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

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
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

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
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

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
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

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
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

   fn is_hashable(&self) -> bool {
      true
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      Some(Box::new(self.clone()))
   }
}
impl Hashable for MkyString {
   fn hash_key(&self) -> HashKey {
      let mut hasher: DefaultHasher = DefaultHasher::new();
      hasher.write(self.value.as_bytes());
      
      HashKey { r#type: self.r#type(), value: hasher.finish() }
   }
}



// Bill Tin LULW
type BuiltInFunction = fn(args: Vec<Box<dyn Object>>) -> Box<dyn Object>;
#[derive(Clone, Debug)] 
pub struct BuiltIn {
   pub func: BuiltInFunction,
}
impl Object for BuiltIn {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::BuiltInObj.to_string()
   }

   fn inspect(&self) -> String {
      "builtin function".into()
   }

   fn as_any(&self) -> &dyn Any {
      self
   }

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
   }
}



#[derive(Clone, Debug)] 
pub struct Array {
   pub elements: Vec<Box<dyn Object>>,
}
impl Object for Array {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::ArrayObj.to_string()
   }

   fn inspect(&self) -> String {
      let mut out: String = String::new();
      let mut el: Vec<String> = Vec::new();

      for e in &self.elements {
         el.push(e.inspect())
      }
      out.push_str("[");
      out.push_str(el.join(", ").as_str());
      out.push_str("]");

      out
   }

   fn as_any(&self) -> &dyn Any {
      self
   }

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
   }
}



#[derive(Clone, Debug)] 
pub struct HashPair {
   pub key: Box<dyn Object>,
   pub value: Box<dyn Object>,
}

#[derive(Clone, Debug)] 
pub struct Hash {
   pub pairs: HashMap<HashKey, HashPair>
}
impl Object for Hash {
   fn r#type(&self) -> ObjectType {
      ObjectTypes::HashObj.to_string()
   }

   fn inspect(&self) -> String {
      let mut out: String = String::new();
      let mut pairs: Vec<String> = Vec::new();

      for (_, pair) in &self.pairs {
         pairs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()))
      }
      out.push_str("{");
      out.push_str(pairs.join(", ").as_str());
      out.push_str("}");

      out
   }

   fn as_any(&self) -> &dyn Any {
      self
   }

   fn is_hashable(&self) -> bool {
      false
   }

   fn downcast_hashable(&self) -> Option<Box<dyn Hashable>> {
      None
   }
}
