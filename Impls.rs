use crate::Structs::SaltStudent;
use crate::Structs::Bike;

impl SaltStudent {
  fn get_name(&self) -> &String {
    return &self.name;
  }
  fn get_age(&self) -> &i8 {
    &self.age
  }
}
