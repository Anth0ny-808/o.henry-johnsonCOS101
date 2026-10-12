use std::io;

fn main() {
    loop {
        println!("\n===== APS Level Checker =====");
        println!("Select Occupation:");
        println!("1. Office Admin");
        println!("2. Academic");
        println!("3. Lawyer");
        println!("4. Teacher");
        println!("=============================");

        println!("Enter occupation number (1-4):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice = choice.trim();

        if choice != "1" && choice != "2" && choice != "3" && choice != "4" {
            println!("Invalid occupation. Please try again.");
            continue;
        }

        println!("Enter years of experience:");
        let mut years_input = String::new();
        io::stdin().read_line(&mut years_input).expect("Failed to read");
        let years: i32 = years_input.trim().parse().expect("Please enter a valid number");

        
        let level;
        let index;

        if years >= 1 && years <= 2 {
            level = "APS 1-2";
            index = 0;
        } else if years >= 3 && years <= 5 {
            level = "APS 3-5";
            index = 1;
        } else if years >= 6 && years <= 8 {
            level = "APS 5-8";
            index = 2;
        } else if years >= 9 && years <= 10 {
            level = "EL1 8-10";
            index = 3;
        } else if years >= 11 && years <= 13 {
            level = "EL2 10-13";
            index = 4;
        } else {
            level = "SES";
            index = 5;
        }

        // Job titles stored in vectors
        let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
        let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
        let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
        let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

        // Get the correct position from the vector
        let position;
        if choice == "1" {
            position = office_admin[index];
        } else if choice == "2" {
            position = academic[index];
        } else if choice == "3" {
            position = lawyer[index];
        } else {
            position = teacher[index];
        }

        println!("\n---------- RESULT ----------");
        if choice == "1" {
            println!("Occupation          : Office Admin");
        } else if choice == "2" {
            println!("Occupation          : Academic");
        } else if choice == "3" {
            println!("Occupation          : Lawyer");
        } else {
            println!("Occupation          : Teacher");
        }

        println!("Years of Experience : {}", years);
        println!("APS Level           : {}", level);
        println!("Position            : {}", position);
        println!("----------------------------");

        //for continuation
        println!("\nDo you want to check another staff? (y/n)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read");

        if again.trim().to_lowercase() == "n" {
            println!("\nThank you. Goodbye!");
            break;
        }
    }
}