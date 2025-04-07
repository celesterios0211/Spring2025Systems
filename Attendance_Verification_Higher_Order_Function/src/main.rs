//Create a struct student (major)
struct Student{
    major: String,
}
//Higer order functions update majors
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student,String)){
    for student in collection.iter_mut() {
        behavior(student, "BMed".to_string());
        print_student(student);
    }
}
//First order function, assign_major(sudent,major_declared), 

fn assign_major(s: &mut Student, major_declared: String){
    s.major = major_declared;
}

fn print_student(student: &Student) {
    println!("Updated major: {}", student.major);
}

 fn main(){
//create a vector of students1,2,3, and update all students major
let students = vec![
    Student { major: "Undeclared".to_string() },
    Student { major: "Chemistry".to_string() },
    Student { major: "Sociology".to_string() },
];

//Print vector of students before updating
for i in &students{
    println!("Major Before Updating: {}", i.major);
}

//Update vector of students
update_majors(students, assign_major);
}