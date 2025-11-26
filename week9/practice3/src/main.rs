use std::io::Write;
use std::fs;


fn main() {
    let mut file = fs::File::create("data.txt").expect("Could not create file");
    println!("File created");

    fs::remove_file("data.txt").expect("counld not delete");
    println!("File is removed");
}
