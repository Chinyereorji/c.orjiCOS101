struct Laptop{
    model:String,
    price:f32
}

impl Laptop{
    fn total_cost(&self,quantity:f32) -> f32 {
        self.price * quantity
    }
}

fn main() {
    let lap1 = Laptop{
        model:String::from("HP Laptop"),
        price:650_000.0
    };
    let lap2 = Laptop{
        model:String::from("IBM Laptop"),
        price:755_000.0
    };
    let lap3 = Laptop{
        model:String::from("Toshiba Laptop"),
        price:550_000.0

    };
    let lap4 = Laptop{
        model:String::from("Dell Laptop"),
        price:850_000.0
    };

    let quantity:f32 = 3.0;
    let sum_for_all = 
    lap1.total_cost(quantity) + lap2.total_cost(quantity) + lap3.total_cost(quantity) + lap4.total_cost(quantity);


    println!("The cost of 3 units of {} is N{}",lap1.model,lap1.total_cost(quantity) );
    println!("The cost of 3 units of {} is N{}",lap2.model,lap2.total_cost(quantity) );
    println!("The cost of 3 unit of {} is N{}",lap3.model, lap2.total_cost(quantity) );
    println!("The cost of 3 unit of {} is N{}",lap4.model,lap4.total_cost(quantity) );
    println!("The total cost of buying 3 laptops from each model is N{}",sum_for_all );    

}
