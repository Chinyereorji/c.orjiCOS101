fn main() {
	//compound interest
let p=510_000_000.0;//principle
let r: f64= 5.0;//rate 
let n: u32 = 3; // years
let A = p * (1.0 - (r/100.0)).powi(n as i32);cd
println!("The compound interest is {}", A);
}