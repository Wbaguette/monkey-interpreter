pub trait TestType {
   fn is_i64(&self) -> bool;
   fn downcast_i64(&self) -> Option<i64>;

   fn is_bool(&self) -> bool;
   fn downcast_bool(&self) -> Option<bool>;

   fn is_string(&self) -> bool;
   fn downcast_string(&self) -> Option<String>;
}

impl TestType for i64 {
   fn is_i64(&self) -> bool {
      true
   }
   fn downcast_i64(&self) -> Option<i64> {
      Some(*self)
   }
   fn is_bool(&self) -> bool {
      false
   }
   fn downcast_bool(&self) -> Option<bool> {
      None
   }
   fn is_string(&self) -> bool {
      false
   }
   fn downcast_string(&self) -> Option<String> {
      None
   }
}

impl TestType for bool {
   fn is_i64(&self) -> bool {
      false
   }
   fn downcast_i64(&self) -> Option<i64> {
      None
   }
   fn is_bool(&self) -> bool {
      true
   }
   fn downcast_bool(&self) -> Option<bool> {
      Some(self.clone())
   }
   fn is_string(&self) -> bool {
      false
   }
   fn downcast_string(&self) -> Option<String> {
      None
   }
}

impl TestType for String {
   fn is_i64(&self) -> bool {
      false
   }
   fn downcast_i64(&self) -> Option<i64> {
      None
   }
   fn is_bool(&self) -> bool {
      false
   }
   fn downcast_bool(&self) -> Option<bool> {
      None
   }
   fn is_string(&self) -> bool {
      true
   }
   fn downcast_string(&self) -> Option<String> {
      Some(self.clone())
   }
}
