fn main() {
let name = "Aisha lawal";
let uni:&str ="Pan-atlantic university";
let addr:&str ="km 52 lekki-Epe Expressway, Ibeju lekki, Lagos";
println!("Name: {}",name );
println!("University:{}, \nAddress: {}",uni,addr );


let department: &'static str = "Computer Science";
let school:&'static str = "School of Science and Technology";
println!("Department:{}, \nSchool: {}",department,school );
}
