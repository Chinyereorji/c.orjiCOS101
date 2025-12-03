fn main() {
let v = vec![10,20,30];
//vector v owns the object in heap

let v2 =v; //moves ownerships to v2
display(v2.clone());//.clone()
//v2 is moved to diplay and v2 is invalidated

println!("In main {:?}",v2 );  
}
fn display(v:Vec<i32>){
    println!("Inside display {:?}",v );
}
