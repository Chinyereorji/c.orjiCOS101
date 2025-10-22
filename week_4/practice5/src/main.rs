//Rust program to read the height of a person
//and then print if person is tall,dwarf,
//or average height person
use std::io;
fn main() {
let mut input = String::new();
println!("\nEnter the Height(in centimeters): ");
io::stdin.read_file(&mut input).expect("not a valid string");
let height:f32 = input.trim().parse().expec("not a valid string");

if height >=150.0 && height <=170.0
{
    println!("You are average height person");
}
else if height >170.0 && height <=195.0
}
println!("You are tall");
else if <150 && height> 100.0
}
println!("you are dwarf");
}
else{
    println!("abnormal height");
}
}
