use std::io;


fn main() {
            println!("The Menu");
println!("Code          Menu                                 Price");
println!("P             Poundo Yam/Edikaiko Soup            N3,200");
println!("F             Fried Rice & Chicken                N3,000");
println!("A             Amala & Ewedu Soup                  N2,500");
println!("E             Eba & Egusi Soup                    N2,000");
println!("W             White Rice and Stew                 N2,500"); 

println!("Please enter the Code of your food");

let mut code= String::new();
io::stdin().read_line(&mut code).expect("Not valid Input");
let code= code.trim().to_uppercase();


println!("Enter the quantity you want to order");

let mut quantity= String::new();
io::stdin().read_line(&mut quantity).expect("Not valid Input");
let quantity:f32 = quantity.trim().parse().expect("Not a valid Input");

let  food;
let  price:f32;

if code== "P" {
    food= "Pounded Yam/Edikaiko Soup";
    price= 3200.0;
}
else if code== "F"{
    food= "Fried Rice & Chicken";
    price= 3000.0;
}
else if code == "A"{
    food="Amala & Ewedu Soup";
    price= 2500.0;

}
else if code == "E"{
    food= "Eba & Egusi Soup";
    price= 2000.0;
}
else if code == "W"{
    food= "White Rice and Stew";
    price= 2500.0;
}
else {
    println!("Not on the menu");
    return;
}
println!("Your food is {}",food );


let  total:f32 = price * quantity;

if total > 10000.00 {
    let total= 0.95 * total;
    println!("You have a 5% discount your total is now {}",total );

}
else {
    println!("Your total price for {} quantity is N{}",quantity,total );


}




}