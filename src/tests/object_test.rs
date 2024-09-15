#[cfg(test)]

use crate::objects::{Hashable, MkyString};


#[test]
fn test_string_hash_key() {
   let hello1: MkyString = MkyString{ value: "Hello World".to_string() };
   let hello2: MkyString = MkyString{ value: "Hello World".to_string() };
   let diff1: MkyString = MkyString{ value: "My name is johnny".to_string() };
   let diff2: MkyString = MkyString{ value: "My name is johnny".to_string() };

   assert_eq!(hello1.hash_key(), hello2.hash_key(), "\nstrings with same content have different hash keys");
   assert_eq!(diff1.hash_key(), diff2.hash_key(), "\nstrings with same content have different hash keys");
   assert_ne!(hello1.hash_key(), diff1.hash_key(), "\nstrings with different content have same hash keys");
}
