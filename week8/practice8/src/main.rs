fn main() {
    //initialize a mutable tuple
    let mut mountain1 = ("Everest",8848,"Fishtail",6993);

    println!("Original Tuple = {:?};"mountain1);

    //Change 3rd and 4th element in a mutable tuple
    mountain1.2 = "Lhotse";
    mountain1.3 = 8516;
    println!("Changed tuple = {:?}",mountain1 );
}
