//Rust Program to find the roots of a quadratic equation 
//given the values a,b,c
use std::io;
fn main() {
    println!("The Value a is:");
    let mut a =String::new();
    io::stdin().read_line(&mut a).expect("not a valid number");
    let a:f32 = a.trim().parse().expect("not a  valid number");

    println!("The Value b is");
      let mut b=String::new();
    io::stdin().read_line(&mut b).expect("not a valid number");
    let b:f32 = b.trim().parse().expect("not a  valid number");

    println!("The value of c is");
   let mut c =String::new();
    io::stdin().read_line(&mut c).expect("not a valid number");
    let c:f32 = c.trim().parse().expect("not a  valid number");

    let discriminant: f32  = b*b - 4.0 * a * c;
    if discriminant >0.0{
    let root1: f32 =( -b + discriminant.sqrt())/(2.0 *a);
    let root2:f32 = (-b -discriminant.sqrt())/(2.0 * a);
    println!("The roots are {} and {}",root1,root2);
}
else if discriminant == 0.0{
    let root1 : f32 = -b/(2.0 * a);
    println!("The one real root is:{}",root1);

}
else{
    println!("No real root");           
}

}
