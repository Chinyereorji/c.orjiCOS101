fn main() {
    //Array of Numbers
    let number = [1,2,3,4,5];
    println!("Original array = {:?}",number );

    //creat a slice of 2nd and 3rd element
    let slice1 = &number[1..3];
    println!("2nd and 3rd element sliced = {:?}",slice1 );

    //omit the start index
    let slice2 = &number[..3];
    //omit the end index

    let slice3 = &number[2..];
 //This means the slice starts from index 0 and goes up to index
    println!("Index 2 to 5 sliced = {:?}",slice3);
    //omit the start index and the end index
    //reference the whole array
    let slice4 = &number[..];
    //let means the slice starts from index 0 and goes up to inedex 5(exclusive)
    println!("Index 0 to Index5 sliced = {:?}",slice4 );
}
