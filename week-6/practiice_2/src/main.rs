use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:i32 = input.trim().parse().expect("Valid input");

    if ch >= 0 && ch <= 9
    {
        print!("Character '{}' us a digit",ch);
    }
    else
    {
        println!("Character '{}' is not a digit",ch);
    }
}

fn main() {
    //calling function 
    println!("welcome! This program checks whether a character variable contains a digit or not");
    checker()
}
