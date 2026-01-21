use std::io::Read;
use std::io;

fn display_table(file: &str) {
    let mut table = std::fs::File::open(file).unwrap();
    let mut content = String::new();
    table.read_to_string(&mut content).unwrap();
    print!("{}",content)

}
fn main() {
    println!("Hi welcome to Globacom ltd");
    println!("What is your position in the company?\n Pick a number ");

    let mut position= String::new();
    println!("1.Administrator\n 2.Project Manager\n 3.Employee\n 4.Customer\n 5.Vendor\n");
    io::stdin().read_line(&mut position).expect("Failed to read input");
    let role:u8 = position.trim().parse().expect("not a valid number");

 match role {
        1 => display_table("globacom_dbase.sql"),
        2 => display_table("project_tb.sql"),
        3 => display_table("staff_tb.sql"),
        4 => display_table("customer_tb.sql"),
        5 => display_table("dataplan_tb.sql"),
        _ => println!("Access denied:Invalid role"),

    }

    
}