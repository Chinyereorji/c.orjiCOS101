fn main() {
    //mutable array
    let mut colors = ["red","green","Yellow","White"];

    println!("\n Original array = {:?}",colors);

    //Mutuable slice
    let sliced_color = &mut colors[1..3];
     println!("First Slice = {:?}",sliced_color );

     // change the value of the original slice at the first
     sliced_color[1] = "purple";
     println!("Changes slice = {:?}",sliced_color );
}
