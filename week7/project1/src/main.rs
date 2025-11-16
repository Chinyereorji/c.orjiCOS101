fn area_trapezium(height:f64, base1:f64, base2:f64) ->f64 {
    return(height/2.0) * (base1 + base2);
}
fn area_Rhombus(dia1:f64,dia2:f64) ->f64{
    return(dia1 * dia2)/2.0 ;

}
fn area_parallelogram(base:f64, alititude:f64) ->f64{
    return base * alititude;
}
fn area_cube(length:f64) ->f64{
    return 6.0 * length*length;
}
fn pi()->f64{
    let a = 22.0;
    let b = 7.0;
    let c = a/b;
    return c;
}
fn volume_cylinder(radius:f64, height:f64)->f64{
    return pi() * radius * radius * height;
}


use std::io;

fn main() {
    println!("Hi User, What would you like to calculate
        1. Area of Area Of Trapazium
        2. Area of Area Of Rhombus
        3. Area of Area Of Parallelogram
        4. Area of Area Of Cube 
        5. Volume of Cylinder ");

    let mut shape= String::new();
    io::stdin().read_line(&mut shape).expect("Not a Valid Input");
    let shape:u8 = shape.trim().parse().expect("Not a valid Input");

    if shape == 1 {
        let height = read_input("Enter the height: ");
        let base1 = read_input("Enter base 1: ");
        let base2 = read_input("Enter base 2: ");

        let area = area_trapezium(height, base1, base2);
        println!("Area of the Trapezium = {}", area);

    }
    else if shape == 2{
        let dia1 = read_input("Enter The first diagonal: ");
        let dia2 = read_input("EnterThe second diagonal: ");

        let area = area_Rhombus(dia1,dia2);
        println!("The Area of the Rhombus is: {}",area );

    }
    else if shape == 3{
        let base = read_input("Enter the Base of the Parallelogram: ");
        let alititude = read_input("Enter the altitude of the Parallelogram: ");
        let area = area_parallelogram(base, alititude);
        println!("The area of the parallelogram is {}",area );
    }
    else if shape == 4{
        let length = read_input("Enter the length of the Square");
        let area = area_cube(length);
    }
    else if shape == 5{
        let radius = read_input("Enter the radius of the cylinder");
        let height = read_input("Enter The Height of the cylinder");
        let volume = volume_cylinder(radius, height);
        println!("The Volume of the cylinder is: {}", volume);

    }
    else {
        println!("Not a Valid Option");
    }

    fn read_input(value: &str) -> f64 {
    println!("{}", value);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");

    return input.trim().parse().expect("Enter a valid number");
}
} 