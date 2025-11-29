use std::io::Write;
use std::fs::File;
fn main() {
    let ministers = vec![
    ("Aigbongun Alamba Daudu","Internal Affairs","South West"),
    ("Muritala Afeez Bendu","Justice","North East",),
    ("Okorocha Calistus Ogbona","Defense","South South"),
    ("Adewale Jimoh Akanbi","Power & Steel","South West"),
    ("Osezuwa Faith Elieye","Petroleum","South East"),
    ];


    let mut file = File::create("convicted_ministers.txt").unwrap();

        file.write_all(
            format!("{:<30} {:<20}  {:<20} \n","Name of Commisioner","Ministry","Geopolitical Zone").as_bytes()).unwrap();
        for (name, ministry, zone)in ministers{
        file.write_all(
                    format!("{:<30} {:<20}  {:<20} \n",name,ministry,zone).as_bytes()).unwrap();
        }


    
}
