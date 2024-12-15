use std::io::Read;

fn main() {
  let commisioner_file = read_file("commisioner.txt");
  let commisioner_vec: Vec<&str> = commisioner_file.trim().split('\n').collect();
  let ministry_file = read_file("ministry.txt");
  let ministry_vec: Vec<&str> = ministry_file.trim().split('\n').collect();
  let geopolitical_file = read_file("geopolitical_zone.txt");
  let geopolitical_vec: Vec<&str> = geopolitical_file.trim().split('\n').collect();

  println!("Convicted Ministers:");
  for i in 0..commisioner_vec.len() {
    println!("{}. {} {} {}", i+1, commisioner_vec[i].trim(), ministry_vec[i].trim(), geopolitical_vec[i].trim());
  }
}

fn read_file(name: &str) -> String {
    let mut file = std::fs::File::open(name).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}