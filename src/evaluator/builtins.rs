use crate::objects::*;
use super::NULL;
use std::collections::HashMap;
use std::process;
use lazy_static::lazy_static;
use color_eyre::owo_colors::OwoColorize;

lazy_static! {
   static ref BUILTINS: HashMap<String, BuiltIn> = {
      let mut map = HashMap::new();
      map.insert("exit".to_string(), BuiltIn { func: exit });

      map.insert("len".to_string(), BuiltIn { func: len });
      map.insert("first".to_string(), BuiltIn { func: first });
      map.insert("last".to_string(), BuiltIn { func: last });
      map.insert("rest".to_string(), BuiltIn { func: rest });
      map.insert("push".to_string(), BuiltIn { func: push });
      map.insert("puts".to_string(), BuiltIn { func: puts });
      map.insert("append".to_string(), BuiltIn { func: append });
      map.insert("insert".to_string(), BuiltIn { func: insert });

      map
   };
}

pub fn lookup_builtins(val: &String) -> Option<BuiltIn> {
   BUILTINS.get(val).cloned()
}

fn exit(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.is_empty() {
      eprintln!("{}", "\nExiting Monkey REPL...".bright_red().bold());
      process::exit(0)
   } else {
      return Box::new(Error::new(format!("'exit()' takes 0 arguments. got={}", args.len())))
   }
}

fn len(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 1 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=1", args.len())))
   }
   let arg: &Box<dyn Object> = args.get(0).unwrap();

   if let Some(mky_string) = arg.as_any().downcast_ref::<MkyString>() {
      return Box::new(Integer { value: mky_string.value.len() as i64 })
   } else if let Some(array) = arg.as_any().downcast_ref::<Array>() {
      return Box::new(Integer { value: array.elements.len() as i64 })
   }  else {
      return Box::new(Error::new(format!("argument to 'len' not supported, got {}", arg.r#type())))
   }
} 

fn first(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 1 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=1", args.len())))
   }
   let arg: &Box<dyn Object> = args.get(0).unwrap();
   
   if let Some(arr) = arg.as_any().downcast_ref::<Array>() {
      if !arr.elements.is_empty() {
         return arr.elements.get(0).unwrap().clone()
      } else {
         return Box::new(NULL)
      }
   } else {
      return Box::new(Error::new(format!("argument to 'first' must be ARRAY, got {}", arg.r#type())))
   }
}

fn last(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 1 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=1", args.len())))
   }
   let arg: &Box<dyn Object> = args.get(0).unwrap();
   
   if let Some(arr) = arg.as_any().downcast_ref::<Array>() {
      if !arr.elements.is_empty() {
         return arr.elements.get(arr.elements.len()-1).unwrap().clone()
      } else {
         return Box::new(NULL)
      }
   } else {
      return Box::new(Error::new(format!("argument to 'last' must be ARRAY, got {}", arg.r#type())))
   }
}

fn rest(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 1 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=1", args.len())))
   }
   let arg: &Box<dyn Object> = args.get(0).unwrap();
   
   if let Some(arr) = arg.as_any().downcast_ref::<Array>() {
      if !arr.elements.is_empty() {
         let new_elements: Vec<Box<dyn Object>> = arr.elements.iter().skip(1).cloned().collect();
         return Box::new(Array { elements: new_elements })
      } else {
         return Box::new(NULL)
      }
   } else {
      return Box::new(Error::new(format!("argument to 'rest' must be ARRAY, got {}", arg.r#type())))
   }
}

fn push(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 2 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=2", args.len())))
   }
   let arg: &Box<dyn Object> = args.get(0).unwrap();
   
   if let Some(arr) = arg.as_any().downcast_ref::<Array>() {
      let mut new_elements: Vec<Box<dyn Object>> = arr.elements.iter().cloned().collect();
      new_elements.push(args.get(1).unwrap().clone());
      return Box::new(Array { elements: new_elements })
   } else {
      return Box::new(Error::new(format!("argument to 'push' must be ARRAY, got {}", arg.r#type())))
   }
}

fn puts(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   for arg in &args {
      println!("{}", arg.inspect())
   }

   Box::new(NULL)
}

fn append(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 2 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=2", args.len())))
   }
   let arr1: &Box<dyn Object> = args.get(0).unwrap();
   let arr2: &Box<dyn Object> = args.get(1).unwrap();

   if let Some(array1) = arr1.as_any().downcast_ref::<Array>() {
      if let Some(array2) = arr2.as_any().downcast_ref::<Array>() {
         let mut cloned_elements_1: Vec<Box<dyn Object>> = array1.elements.clone();
         let mut cloned_elements_2: Vec<Box<dyn Object>> = array2.elements.clone();
         cloned_elements_1.append(&mut cloned_elements_2);

         return Box::new(Array { elements: cloned_elements_1 })
      } else {
         return Box::new(Error::new(format!("second argument to 'sort' must be ARRAY, got {}", arr2.r#type())))
      }
   } else {
      return Box::new(Error::new(format!("first argument to 'sort' must be ARRAY, got {}", arr1.r#type())))
   }
}

// A new map is always created in memory and returned, this is because of mutable borrowing in Rust
fn insert(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
   if args.len() != 3 {
      return Box::new(Error::new(format!("wrong number of arguments. got={}, want=3", args.len())))
   }
   let map_to_insert_into: &Box<dyn Object> = args.get(0).unwrap();

   let key_to_insert: &Box<dyn Object> = args.get(1).unwrap();
   let value_to_insert: &Box<dyn Object> = args.get(2).unwrap();

   // Here we cannot mutate map.pairs because we cannot borrow data in a '&' reference as mutable
   if let Some(map) = map_to_insert_into.as_any().downcast_ref::<Hash>() {
      if !key_to_insert.is_hashable() {
         return Box::new(Error::new(format!("second argument to 'insert' must be hashable, got {}", key_to_insert.r#type())))
      }
      let hash_key: HashKey = key_to_insert.downcast_hashable().unwrap().hash_key();
      let hash_pair: HashPair = HashPair { key: key_to_insert.clone(), value: value_to_insert.clone() };

      let mut copied_map: HashMap<HashKey, HashPair> = map.pairs.clone();
      copied_map.insert(hash_key, hash_pair);
      Box::new(Hash { pairs: copied_map })
   } else {
      return Box::new(Error::new(format!("first argument to 'insert' must be HASH, got {}", map_to_insert_into.r#type())))
   }
}
