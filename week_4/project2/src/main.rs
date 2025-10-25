
use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Do you have any experience? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("not a valid answer");
    let experience = experience.trim().to_lowercase();
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("not a valid input");
    let age: u8 = age.trim().parse().expect("Not a valid age");

    println!("My age is {}", age);
    let incentive: u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_300_000; 
        }
    } else {
        incentive = 100_000;
    }

    println!("The employee incentive is N{}", incentive);
}
