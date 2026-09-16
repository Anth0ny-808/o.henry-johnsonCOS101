use std::io;
fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: " );
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");


    println!("Enter the value of b: " );
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c : " );
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");


    
    let d:f32 = b*b - 4.0*a*c;

    println!("the discriminant is {}", d);

    if d > 0.0 as f32 
    {
        println!("two distinct roots:");
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2); 
    }
    
    else if d == 0.0 as f32
    {
        println!("exactly one root:");
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    }
    else
    {
        println!("no real roots:");
    }

}


