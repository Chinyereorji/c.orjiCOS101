fn main() {
    //Name Vector
    let name = vec!["Mary","Sam","Sally","Gregs","Ade","Mark","June","Ife"];

    //Age vector
    let age= vec![16,17,18,19,20,21,18,23];

    println!("\nAge allocation:\n");

    //loop to iterate elements in vector

    for i in 0..age.len()
    {
        //iteration through i on the vector
        println!("{} is {} years old\n",name[i],age[i] );
    }
    
}
