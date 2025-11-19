fn main() {
    let f = vec!['c','o','m','p','u','t','e','p'];
    let mut in1 = String::new();

    println!("Enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut in1).expect("Failed to read input");

    //index is the non negativ(e value which is samaller than the size of the vector
    let index:usize = in1.trim().parse().expect("Not a vaild Input");

    //getting the  valbue at given index value
    let ch: char = v[index];

        println!("{} is the character for the index [{}]\n",ch, index);
    
}
