use std::collections::HashMap;
use crate::objects::Object;

#[derive(Clone, Debug)]
pub struct Environment {
   pub store: HashMap<String, Box<dyn Object>>,
   pub outer: Option<Box<Environment>>,
}
impl Environment {
   pub fn new() -> Self {
      let e: Environment = Environment {
         store: HashMap::new(),
         outer: None,
      };
      e
   }

   pub fn new_enclosed_env(outer: Environment) -> Self {
      let mut e: Environment = Self::new();
      e.outer = Some(Box::new(outer.clone()));
      e
   }

   pub fn get(&self, name: &str) -> Option<&Box<dyn Object>> {    
      let mut found: Option<&Box<dyn Object>> = self.store.get(name);
      // If we can't find a binding in this current environment, and we have an associated outer environment, 
      //    check for that binding there
      if found.is_none() && self.outer.is_some() {
         found = self.outer.as_ref().unwrap().get(name);
      }
      found
   }

   // Here Option::None if the key didnt exist, 
   //  If key did exist it gets updated and returns Some("old_object")
   pub fn set(&mut self, name: &str, val: Box<dyn Object>) -> Option<Box<dyn Object>> {
      self.store.insert(name.to_string(), val)       
   }
}
