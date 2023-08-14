#[allow(unused)]

use crate::objects::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
   static ref BUILTINS: HashMap<String, BuiltIn> = {
      let mut map = HashMap::new();
      map.insert("len".to_string(), BuiltIn { func: len });


      map
   };
}

pub fn lookup_builtins(val: &String) -> Option<BuiltIn> {
   BUILTINS.get(val).cloned()
}

fn len(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 1 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=1", args.len())))
   }

   let arg: &Box<dyn Object> = args.get(0).unwrap();
   if let Some(mky_string) = arg.as_any().downcast_ref::<MkyString>() {
      return Box::new(Integer { value: mky_string.value.len() as i64 })
   } else {
      return Box::new(Error::new(format!("argument to 'len' not supported, got {}", arg.r#type())))
   }
} 
