use std::io::Write;
use std::fs::File; 

fn main() {
    let lager = vec![
    "33 Export",
    "Desperado",
    "Gulder",
    "Heineken",
    "Star"];
    let stout = vec![
    "Legend",
    "Turbo King",
    "Williams"];
    let non_alcoholic = vec![
    "Maltina",
    "Amstel Malta",
    "Malta Gold",
    "Fayrouz"];


    let mut file = File::create("Nigerian_Brewery_Limited.txt").unwrap();
    println!("File creation Successfull");
    file.write_all("Welcome to the Nigerian Brewery Limited\nThese are a list of our services"
    .as_bytes()).unwrap();
    file.write_all(" \nLager: \n".as_bytes()).expect("write failed");
    for items in 0..lager.len(){
         file.write_all(format!("-{} \n", lager[items]).as_bytes()).expect("write failed");
}

    file.write_all("Stout :\n".as_bytes()).expect("write failed");
    for items in 0..stout.len(){
    file.write_all(format!("-{} \n", stout[items]).as_bytes()).expect("write failed");
}
    file.write_all("Non Alcoholic :\n ".as_bytes()).expect("Write Failed");
    for items in 0..non_alcoholic.len(){
     file.write_all(format!("-{} \n", non_alcoholic[items]).as_bytes()).unwrap();

    }

    
}
