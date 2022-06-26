#[derive(Debug)]
pub struct SaltStudent {
  name: String,
  age: i8,
  background: String,
  course: String,
  tests_passed: i8,
  tests_failed: i8,
  final_project_name: String,
  overall_grade: i8,
  bike: Bike
}

#[derive(Debug)]
pub struct Bike {
  name: String,
  tipology: String
}
