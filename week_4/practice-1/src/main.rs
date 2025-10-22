//Rust Program to output names and aged
  use std::io;
fn main() {
  println!("\nStudents Information Management System!");

  //Input name
  print!("\n Please enter you name", );
  let mut name = String::new();
  io::stdin()
  .read_line(&mut name)
  .expect("faild to read input");
  println!("Your name is {}",name );

  //Input age
  println!("\nEnter your age");
  let mut age = String::new();
 io::stdin().read_line(&mut age).expect("failed to read input");
 let age:i32 = age.trim().parse().expect("input not an integer");
println!("Your name is: {}",age );
}
