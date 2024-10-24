mod student;
use student::Student;


fn main() {
   let name = "John".to_string();
   let major = "Computer Science".to_string();
   
   let s = Student::new_student(name,major);

   println!("{:?}", s);

   s.introduce_yourself();
}

