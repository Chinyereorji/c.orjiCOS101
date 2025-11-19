fn main() {
    //Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //Printing The size of Vector
    println!("\nThe length of Vec::new is:{}",v.len() );

    //Using Micro
    let v = vec!["Grace","Effiong","Basil","Kareen","Susan"];

    //printing the size of vector
    println!("\n The length of vec macro is: { }", v.len());
}
