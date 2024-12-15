use std::io::Write;

fn main() {
    let lager_vec = vec!["33 Export\n","Desporados\n","Goldberg\n","Gulder\n","Heineken\n","Star\n"];
    let stout_vec = vec!["Legend\n","Turbo king\n","Williams\n"];
    let non_alcoholic_vec = vec!["Maltina\n","Amstel Malta\n","Malta Gold\n","Fayrouz\n"];
    save_vec("larger.txt", lager_vec);
    save_vec("stout.txt", stout_vec);
    save_vec("non_alcoholic.txt", non_alcoholic_vec);
}

fn save_vec(name: &str, vec: Vec<&str>) {
    
let mut file = std::fs::File::create(name).expect("create failed");
for item in vec.iter(){
    file.write_all(item.as_bytes()).expect("Write failed");
}
println!("\nData written to file.");

}
