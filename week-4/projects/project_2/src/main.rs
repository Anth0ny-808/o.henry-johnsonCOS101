use std::io;
fn main(){

    println!("Get exprience status");
    let mut input1 = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced = input1;

    println!("Enter employee age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age_input:f32 = input2.trim().parse().expect("Not a valid number");

    
     if experienced == "yes" {
        if age_input >= 40.00 {
            println!(" incentive = 1_560_000.00");
        } else if age_input >= 30.00 && age_input <= 39.00 {
            println!("incentive = 1_480_000.00");
        } else if age_input <= 29.00 {
            println!("incentive = 1_300_000.00"); }
     } 
     else { println!("incentive = 100_000.00"); }








    

}     