use std::any::Any;


#[derive(Debug, Clone)]
pub enum ObjectTypes {
   IntegerObj,
   BooleanObj,
   NullObj,
}
impl ObjectTypes {
   pub fn to_string(&self) -> String {
      return match self {
         Self::IntegerObj => "Integer",
         Self::BooleanObj => "Boolean",
         Self::NullObj => "NULL",
      }.to_string()
   }
}



type ObjectType = String;
pub trait Object: Sync {
   fn r#type(&self) -> ObjectType;
   fn inspect(&self) -> String;
   fn as_any(&self) -> &dyn Any;
}
impl std::fmt::Debug for dyn Object {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(format!("{}", self.inspect()).as_str())
   }
}
impl PartialEq for dyn Object {
   fn eq(&self, other: &Self) -> bool {
      self.r#type() == other.r#type() && self.inspect() == other.inspect()
   }
   fn ne(&self, other: &Self) -> bool {
      self.r#type() != other.r#type() || self.inspect() != other.inspect()
   }
}



#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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