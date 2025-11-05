fn main() {
   let name1 = "Ayomide Adesokan";
   println!("My name is {}",name1 );


   //find and replace
   let name2 = name1.replace("Ayomide","Adebare");
   println!("You can also call me {}",name2 );
   let faculty ="facultyof Science and Technology";


   //find and replace
   let school = faculty.replace("Faculty", "Science");
   println!("I am a Student of the {}",school );
}
