 fn main() {
let Numbers= [450_000.0,1_500_000.0,750_000.0,2_850_000.0,250_000.0];
let Quantity=[2.0,1.0,3.0,3.0,1.0];
let Total: f64= Numbers.iter().sum();
let Sqty: f64= Quantity.iter().sum();//sum of quantity
let Average= Total/Sqty as f64;
println!("The sum is {}", Total);
println!("The average is {}", Average); 
 }