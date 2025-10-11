fn main() {
	//compound interest
let p=520_000_000.0;//principle
let r: f64= 10.0;//rate 
let n: u32 = 5; // years
let A = p * (1.0 + (r/100.0)).powi(n as i32);
let CI= A-p;
println!("The compound interest is {}", CI);
}