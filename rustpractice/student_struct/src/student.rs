// provide 3 methods
// create a new Student
// change name
// change major
#[derive(Debug)]
struct Student {
    name: String,
    major: String,
}

  impl Student {
    fn new_student(name:String, major:String) -> Self {
        Student{
            name:name,
            major:major,
        }
    }
    fn introduce_yourself(&self) {
        println!("Hey my name is {}. I am majoring in {}", self.name, self.major);
      }
    ///fn change_major(&mut self, new_major
      ///self.major = new_major.to_string();
    }

  

fn main() {
   let name = "John".to_string();
   let major = "Computer Science".to_string();
   
   let s = Student::new_student(name,major);

   println!("{:?}", s);

   s.introduce_yourself();
}
