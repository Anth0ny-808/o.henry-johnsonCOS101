use std::io;
use std::f64::consts::PI;

fn main() {
    loop {
        println!("\n===== Shape Calculator =====");
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Surface Area");
        println!("5. Cylinder Volume");
        println!("6. Exit");
        println!("============================");

        println!("Enter your choice:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice = choice.trim();

        if choice == "1" {
            println!("\nEnter height:");
            let mut h = String::new();
            io::stdin().read_line(&mut h).expect("Failed");
            let height: f64 = h.trim().parse().expect("Enter a number");

            println!("Enter first base:");
            let mut b1 = String::new();
            io::stdin().read_line(&mut b1).expect("Failed");
            let base1: f64 = b1.trim().parse().expect("Enter a number");

            println!("Enter second base:");
            let mut b2 = String::new();
            io::stdin().read_line(&mut b2).expect("Failed");
            let base2: f64 = b2.trim().parse().expect("Enter a number");

            let area = height / 2.0 * (base1 + base2);
            println!("Area of Trapezium is {:.2}", area);
        }
        else if choice == "2" {
            println!("\nEnter first diagonal:");
            let mut d1 = String::new();
            io::stdin().read_line(&mut d1).expect("Failed");
            let diagonal1: f64 = d1.trim().parse().expect("Enter a number");

            println!("Enter second diagonal:");
            let mut d2 = String::new();
            io::stdin().read_line(&mut d2).expect("Failed");
            let diagonal2: f64 = d2.trim().parse().expect("Enter a number");

            let area = 0.5 * diagonal1 * diagonal2;
            println!("Area of Rhombus is {:.2}", area);
        }
        else if choice == "3" {
            println!("\nEnter base:");
            let mut b = String::new();
            io::stdin().read_line(&mut b).expect("Failed");
            let base: f64 = b.trim().parse().expect("Enter a number");

            println!("Enter altitude:");
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("Failed");
            let altitude: f64 = a.trim().parse().expect("Enter a number");

            let area = base * altitude;
            println!("Area of Parallelogram is {:.2}", area);
        }
        else if choice == "4" {
            println!("\nEnter side of the cube:");
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Failed");
            let side: f64 = s.trim().parse().expect("Enter a number");

            let surface = 6.0 * side * side;
            println!("Surface Area of Cube is {:.2}", surface);
        }
        else if choice == "5" {
            println!("\nEnter radius:");
            let mut r = String::new();
            io::stdin().read_line(&mut r).expect("Failed");
            let radius: f64 = r.trim().parse().expect("Enter a number");

            println!("Enter height:");
            let mut h = String::new();
            io::stdin().read_line(&mut h).expect("Failed");
            let height: f64 = h.trim().parse().expect("Enter a number");

            let volume = PI * radius * radius * height;
            println!("Volume of Cylinder is {:.2}", volume);
        }
        else if choice == "6" {
            println!("\nThank you. Goodbye!");
            break;
        }
        else {
            println!("Invalid choice. Please try again.");
        }
    }
}
