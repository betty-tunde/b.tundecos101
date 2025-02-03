fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let add:&str = "km 52 Lekki-epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni, add);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of science and Technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
