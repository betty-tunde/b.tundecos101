
   use std::io;

fn main() {
    // input needed
    let mut name = String::new();
    let mut email = String::new();
    let mut department = String::new();
    let mut state_of_origin = String::new();
    let mut cgpa_input = String::new();
    let mut class_level_input = String::new();
    let mut class_rep_input = String::new();

   
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().string(); 

    println!("Enter your email:");
    io::stdin().read_line(&mut email).expect("Failed to read line");
    let email = email.trim().string();

    println!("Enter your department:");
    io::stdin().read_line(&mut department).expect("Failed to read line");
    let department = department.trim().string();

    println!("Enter your state of origin:");
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read line");
    let state_of_origin = state_of_origin.trim().string();

    println!("Enter your CGPA (eg, 4.5):");
    io::stdin().read_line(&mut cgpa_input).expect("Failed to read line");
    let cgpa: f32 = cgpa_input.trim().parse().expect("Re-enter number");

    println!("Enter your class level (eg, 100, 200, 300):");
    io::stdin().read_line(&mut class_level_input).expect("Failed to read line");
    let class_level: u32 = class_level_input.trim().parse().expect("Re-enter number");

    println!("Are you a Class Rep? (yes/no):");
    io::stdin().read_line(&mut class_rep_input).expect("Failed to read line");
    let is_class_rep = class_rep_input.trim() "yes";

   


  
if student_is_a_class_rep_and_their_class_level != 100 & cgpa > 4.0 {
        
        println!("\nYou can vote!");
        println!("Name: {}", name);
        println!("Email: {}", email);
        println!("Department: {}", department);
        println!("State of Origin: {}", state_of_origin);
    } else {
        
        println!("\nCan't Vote");
    }



}

