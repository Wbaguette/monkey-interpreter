use std::collections::HashMap;

use crate::objects::Object;

#[derive(Clone, Debug)]
pub struct Environment {
   pub store: HashMap<String, Box<dyn Object>>,
}
impl Environment {
   pub fn new() -> Self {
      let e: Environment = Environment {
         store: HashMap::new()
      };

      e
   }

   pub fn get(&self, name: &str) -> Option<&Box<dyn Object>> {
      self.store.get(name)
   }

   pub fn set(&mut self, name: &str, val: Box<dyn Object>) -> Option<Box<dyn Object>> {
      self.store.insert(name.to_string(), val)        // Here Option::None if the key didnt exist, If key did exist it gets updated and returns the old Object
   }
}
