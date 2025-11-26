use std::io;
fn main() {
    let mut candidate:Vec<(String, u8)> = Vec::new();
    println!("How many candidates do you want to input");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Not valid input");
    let input:u32 = input_1.trim().parse().expect("Enter a Number");
    for x in 1..=input{
        println!("Enter the candidates{} name: ", x);
        let mut name =  String::new();
        io::stdin().read_line(&mut name).expect("Not a valid input");
        let name= name.trim().to_string();

        let mut year= String::new();
        println!("How many years of experience do you have?");
        io::stdin().read_line(&mut year).expect("Not a valid Input");
        let years_of_experience:u8= year.trim().parse().expect("Input an Integer");
        candidate.push((name, years_of_experience));
    }
    let mut heighest_experience:(String, u8) = ("".to_string(),0);
    for (name, years_of_experience) in candidate{
        if years_of_experience > heighest_experience.1{
            heighest_experience = (name,years_of_experience);
        }


    }
    println!("The candidate with thje heighest experience is");
    println!("Name:{}",heighest_experience.0 );
    println!("Years of experience:{} ", heighest_experience.1);



    }


 
