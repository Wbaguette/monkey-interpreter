pub mod environment;
use std::{any::Any, fmt::Debug};

#[derive(Debug, Clone)]
pub enum ObjectTypes {
   IntegerObj,
   BooleanObj,
   NullObj,
   ReturnValObj,
   ErrorObj,
}
impl ObjectTypes {
   pub fn to_string(&self) -> String {
      return match self {
         Self::IntegerObj => "INTEGER",
         Self::BooleanObj => "BOOLEAN",
         Self::NullObj => "NULL",
         Self::ReturnValObj => "RETURN_VALUE",
         Self::ErrorObj => "ERROR",
      }.to_string()
   }
}

type ObjectType = String;
pub trait Object {
   fn r#type(&self) -> ObjectType;
   fn inspect(&self) -> String;
   fn as_any(&self) -> &dyn Any;
}
impl Debug for dyn Object {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(self.inspect().as_str())
   }
}

pub fn match_object_to(obj: &Box<dyn Object>) -> Box<dyn Object> {
   if let Some(int_obj) = obj.as_any().downcast_ref::<Integer>() {
      return Box::new( Integer { value: int_obj.value.clone() })
   } else if let Some(bool_obj) = obj.as_any().downcast_ref::<Boolean>() {
      return Box::new( Boolean { value: bool_obj.value.clone() })
   } else if let Some(return_obj) = obj.as_any().downcast_ref::<ReturnValue>() {
      // need some recursive shit here no???
      let return_val = match_object_to(&return_obj.value);
      return Box::new( ReturnValue { value: return_val })
   } else if let Some(error_obj) = obj.as_any().downcast_ref::<Error>() {
      return Box::new( Error { message: error_obj.message.clone() })
   } else {
      return Box::new( Null { })
   }
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
impl PartialEq for ReturnValue {
   fn eq(&self, other: &Self) -> bool {
      self.r#type() == other.r#type() && self.inspect() == other.inspect()
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