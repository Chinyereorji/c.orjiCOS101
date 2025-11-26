use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    

    let mut file = std::fs::File::create("data.txt").expect("create failed");
   file.write_all("Welcome to my world".as_bytes()).expect("write failed");


    let mut append = OpenOptions::new().append(true).open("data.txt").expect("Cannot open file");
    file.write_all("\nHello class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("Write failed");
    println!("file append success");


   
}
