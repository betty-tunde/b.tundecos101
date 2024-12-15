//
use std::io;

//ask if the student is a Class rep

fn main(){
   let mut input = String::new();

   println!("\nAre you a class rep");
   io::stdin().read_line(&mut input).expect("Not a valid string");
   let class_rep = input .trim().parse().expect("Not a valid string");

   //ask if the student is in 100lvl
   if student is in 100lvl >= Yes {
   	println!("Oops, you're not eligile {}", input);
   } else {
   	println!("You're eligible to continue {}" input);
   }
   
}

