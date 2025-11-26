use std::io::Write;
use std::io::Read;
fn main() {
    let message = "Hi welcome to my world\n";

    let mut file = std::fs::File::create("Welcome_message.txt").expect("create failed");
    file.write_all(message.as_bytes()).expect("write failed");
    println!("\n Data written to file");

    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}
