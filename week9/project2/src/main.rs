use std::fs::File;
use std::io::Write;
fn main() {
    let pau_sims = vec![
    ("Oluchi Mordi","ACC10211111","Accounting","300"),
    ("Adams Aliyu","ECO10110101","Ecomomics","100"),
    ("Shania Bolade","CSC10328828","Computer","200"),
    ("Adekunle Gold","EEE11020202","Electrical","200"),
    ("Blanca Edemoh","MEE10202001","Mechanical","100"),
 ];
 
    let mut file =  File::create("pau_sims.txt").unwrap();
    println!("file created successfully");
    file.write_all("PAU SMIS\n".as_bytes()).unwrap();

    
    file.write_all(
        format!("{:<20} {:<15} {:<15} {:<5}\n",
            "Student Name",  "Matric Number", "Department", "Level"
        ).as_bytes()).unwrap();

   
    for (name, matric, department, level) in pau_sims {
        file.write_all(
            format!("{:<20} {:<15} {:<15} {:<5}\n",
                name,matric,department,level).as_bytes()).unwrap();
    }
}
