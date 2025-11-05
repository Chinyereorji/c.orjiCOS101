fn main() {
    let fullname = "Pan-Alantic University";
    println!("Name: {}",fullname );
    println!();
    println!("Before trim");
    println!("Length is {}",fullname.len() );
    println!();
    println!("After trim");
    println!("length is {}",fullname.trim().len() );
}
