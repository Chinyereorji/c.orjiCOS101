fn main() {
   let A : i32 = 10;
   let B : = 20;
    println!("Value of A: {}", A );
    println!("Value of B: {}", B );

    let mut res = A>B;
    println!("A greater than B: {} ", res);

    res = A<B;
    println!("A is lesser than B:{}",res );

    res =A>=B; 
    println!("A greater than or Equal to B: {}",res );

    res = A<=B;
    println!("A lesser than or equal to B: {}",res);

    res = A==B;
    println!("A is equal to B: {}",res );

    res= A!=B;
    println!("A is not equal to B: {}",res );
}


