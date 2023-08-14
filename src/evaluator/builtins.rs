#[allow(unused)]

use crate::objects::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
   static ref BUILTINS: HashMap<String, BuiltIn> = {
      let mut map = HashMap::new();
      map.insert("len".to_string(), BuiltIn { 
         func: || {
            return Box::new(Null)
         },
         
      });

      map
   };
}
