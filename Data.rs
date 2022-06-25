use crate::Structs::SaltStudent;
use crate::Structs::Bike;

pub fn data_stack()-> &Vec<&SaltStudent> {
  let vlad = SaltStudent {
  name: String::from("Vlad"), 
  age: 32,
  background: String::from("C / C++"),
  course: String::from("C#"),
  tests_passed: 8,
  tests_failed: 0,
  final_project_name: String::from("EL PROJECTO!"),
  overall_grade: 94,
  bike: Bike {
    name: String::from("Pegasus"), 
    tipology: String::from("Oldie macho bike")}
  };


let eirik = SaltStudent {
  name: String::from("Eirik"), 
  age: 33,
  background: String::from("Ruby"),
  course: String::from("C#"),
  tests_passed: 8,
  tests_failed: 0,
  final_project_name: String::from("EL PROJECTO!"),
  overall_grade: 94,
  bike: Bike {
    name: String::from("Random Swedish Bike"),
    tipology: String::from("Beta multiple-gear generic bike")
  }
};

let mut student_list: Vec<&SaltStudent> = vec![];

student_list.push(&vlad);
student_list.push(&eirik);

return student_list;
}
