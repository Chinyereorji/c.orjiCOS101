struct Rectangle{
    width:u32, 
    height:u32
    
}
//Logic to Calculate area of the rectangle
impl Rectangle {
    fn area(&self) ->u32{
        self.width * self.height
    }
}
fn main() {
  //instantiating the structure
  let small = Rectangle{
    width:10,
    height:20
  };
  println!("widthis {} \n height is {} \n area of triangle is {}",small.width,small.height,small.area() );
}
