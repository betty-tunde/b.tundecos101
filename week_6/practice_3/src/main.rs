fn main() {
    let name1 = "Ayomide Adesokan";
    println!("my name is {}", name1);

    //find and replace
    let name2 = name1.replace("Ayomide","Adebare");
    println!("you can alos call me {}", name2);
    let faculty = "Faculty of Science and technology";

    //find and replace
    let school = faculty.replace("Faculty","School");
    println!("I am a student of the {}", school);
}
