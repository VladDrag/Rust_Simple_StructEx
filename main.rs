mod structs;

fn main() {
  let vlad = structs::SaltStudent {
    name: String::from("Vlad"), 
    age: 32,
    background: String::from("C / C++"),
    course: String::from("C#"),
    tests_passed: 8,
    tests_failed: 0,
    final_project_name: String::from("EL PROJECTO!"),
    overall_grade: 94,
    bike: structs::Bike {
      name: String::from("Pegasus"), 
      tipology: String::from("Oldie macho bike")}
    };
  
  
  let eirik = structs::SaltStudent {
    name: String::from("Eirik"), 
    age: 33,
    background: String::from("Ruby"),
    course: String::from("C#"),
    tests_passed: 8,
    tests_failed: 0,
    final_project_name: String::from("EL PROJECTO!"),
    overall_grade: 94,
    bike: structs::Bike {
      name: String::from("Random Swedish Bike"),
      tipology: String::from("Beta multiple-gear generic bike")
    }
  };
  
  let mut student_list: Vec<&structs::SaltStudent> = vec![];
  
  student_list.push(&vlad);
  student_list.push(&eirik);

    println!(
        "Studet's Listname is {} and the age is {}. The 
 student rides a {}.",
        &student_list[1].name, &student_list[1].age, &student_list[1].bike.tipology
    );
