use std::io;
fn main() {
    let job_level: Vec<(&str,&str,&str,u8,u8)> = vec![
    //Office admin
    ("Office Administrator","Intern","Aps 1-2",0,2),
    ("Office Administrator","Administrator","Aps 3-5",3,5),
    ("Office Administrator", "Senior Administrator", "APS 5-8", 6, 8),
    ("Office Administrator", "Office Manager", "EL1 8-10", 9, 10),
    ("Office Administrator", "Director", "EL2 10-13", 11, 13),
     ("Office Administrator", "CEO", "SES", 14, 14),
   



    //Academic
    ("Academic"," ","Aps 1-2",0,2),
    ("Academic","Reasearch Assistant","Aps 3-5",3,5),
    ("Academic","Phd candidate","Aps 5-8",6,8),
    ("Academic","Post-Doc Researcher","El1 8-10",9,10),
    ("Academic","Senior Lecturer","El2 10-13",11,13),
    ("Academic","Dean","SES",14,14),

    //Lawyer
    ("Lawyer","Paralegal","Aps 1-2",0,2),
    ("Lawyer","Junior Asspsiate","Aps 3-5",3,5),
    ("Lawyer","Associate","Aps 5-8",6,8),
    ("Lawyer","Senior Associate 1-2","EL1 10-13",11,13),
    ("Lawyer","Partner","SES",14,14),


    // TEACHER
     ("Teacher", "Placement", "APS 1-2", 0, 2),
     ("Teacher", "Classroom Teacher", "APS 3-5", 3, 5),
     ("Teacher", "Snr Teacher", "APS 5-8", 6, 8),
     ("Teacher", "Leading Teacher", "EL1 8-10", 9, 10),
     ("Teacher", "Deputy Principal", "EL2 10-13", 11, 13),
     ("Teacher", "Principal", "SES", 14, 14),];

     println!("What Job do you do (Lawyer,Teacher,Office Administrator,Academics");
     let mut job = String::new();
     io::stdin().read_line(&mut job).expect("Not a Valid Response");
     let role = job.trim();

     println!("How many Yearss Experience do you have?");
     let mut year = String::new();
     io::stdin().read_line(&mut year).expect("Not Valid Input");
     let experience:u8 = year.trim().parse().expect("Not Valid Number");
     let mut found = false;


     for (office,title, band, min_exp, max_exp) in job_level{

        if office.eq_ignore_ascii_case(role) && experience >= min_exp && experience <= max_exp{
            println!("Your role is {}",role );
            println!("Your Title is {}",title );
            println!("Your experience level is {}",band );
            found = true;
            break;
        }
         if !found {
        println!("No matching APS band found for your role and experience.");
    }
    }
        



}
