fn main() {
    let fullname = "Chbudum John umeh";
    let department = "computer science";
    let uni = "Pan Atlantic Univerity";


    let mut school = "School of science".to_string();
    //push string
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
    //check length
    println!("The lenght of fullname is {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);

}
