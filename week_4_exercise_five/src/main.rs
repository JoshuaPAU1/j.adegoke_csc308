use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student{
    fn new (name: String, score: f32) -> Student {
        Student { name, score }
    }
    fn has_passed(&self) -> bool {
        self.score >= 50.0
    }
    fn display_pass_status(&self) {
        if self.has_passed() {
            println!("{} has passed.", self.name);
        } else {
            println!("{} has not passed.", self.name);
        }
    }
}

fn main() {
    let mut name_input = String::new();
    let mut score_input = String::new();

    println!("Enter student's name:");
    io::stdin().read_line(&mut name_input).expect("Failed to read name");
    println!("Enter student's score:");
    io::stdin().read_line(&mut score_input).expect("Failed to read score");

    let name = name_input.trim().to_string();
    let score: f32 = score_input.trim().parse().expect("Please enter a valid number");

    let student = Student::new(name, score);
    student.display_pass_status();
}
