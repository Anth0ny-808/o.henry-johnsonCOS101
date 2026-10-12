use std::io;

fn main() {
    loop {
        println!("\n===== APS Level Checker =====");
        println!("Select Profession:");
        println!(" Public Servant");
        println!("1. Office Admin");
        println!("2. Academic");
        println!("3. Lawyer");
        println!("4. Teacher");
        println!("=============================");

        println!("Enter profession number (1-4):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice = choice.trim();

        if choice != "1" && choice != "2" && choice != "3" && choice != "4"  {
            println!("Invalid profession. Please try again.");
            continue;
        }

        println!("Enter years of experience:");
        let mut years_input = String::new();
        io::stdin().read_line(&mut years_input).expect("Failed to read");
        let years: i32 = years_input.trim().parse().expect("Please enter a valid number");

        // Determine APS Level
        let level;
        if years >= 1 && years <= 2 {
            level = "APS 1-2";
        } else if years >= 3 && years <= 5 {
            level = "APS 3-5";
        } else if years >= 6 && years <= 8 {
            level = "APS 5-8";
        } else if years >= 9 && years <= 10 {
            level = "EL1 8-10";
        } else if years >= 11 && years <= 13 {
            level = "EL2 10-13";
        } else {
            level = "SES";
        }

        // Get position without using vectors
        let position;

        
        if choice == "1" { // Office Admin
            if level == "APS 1-2" {
                position = "Intern";
            } else if level == "APS 3-5" {
                position = "Administrator";
            } else if level == "APS 5-8" {
                position = "Senior Administrator";
            } else if level == "EL1 8-10" {
                position = "Office Manager";
            } else if level == "EL2 10-13" {
                position = "Director";
            } else {
                position = "CEO";
            }
        }
        else if choice == "2" { // Academic
            if level == "APS 1-2" {
                position = "-";
            } else if level == "APS 3-5" {
                position = "Research Assistant";
            } else if level == "APS 5-8" {
                position = "PhD Candidate";
            } else if level == "EL1 8-10" {
                position = "Post-Doc Researcher";
            } else if level == "EL2 10-13" {
                position = "Senior Lecturer";
            } else {
                position = "Dean";
            }
        }
        else if choice == "3" { // Lawyer
            if level == "APS 1-2" {
                position = "Paralegal";
            } else if level == "APS 3-5" {
                position = "Junior Associate";
            } else if level == "APS 5-8" {
                position = "Associate";
            } else if level == "EL1 8-10" {
                position = "Senior Associate 1-2";
            } else if level == "EL2 10-13" {
                position = "Senior Associate 3-4";
            } else {
                position = "Partner";
            }
        }
        else { // Teacher
            if level == "APS 1-2" {
                position = "Placement";
            } else if level == "APS 3-5" {
                position = "Classroom Teacher";
            } else if level == "APS 5-8" {
                position = "Senior Teacher";
            } else if level == "EL1 8-10" {
                position = "Leading Teacher";
            } else if level == "EL2 10-13" {
                position = "Deputy Principal";
            } else {
                position = "Principal";
            }
        }

        // Display result
        println!("\n---------- RESULT ----------");
        
        if choice == "1" {
            println!("Profession          : Office Admin");
        } else if choice == "2" {
            println!("Profession          : Academic");
        } else if choice == "3" {
            println!("Profession          : Lawyer");
        } else {
            println!("Profession          : Teacher");
        }

        println!("Years of Experience : {}", years);
        println!("APS Level           : {}", level);
        println!("Position            : {}", position);
        println!("----------------------------");

        
        println!("\nDo you want to check another staff? (y/n)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read");

        if again.trim().to_lowercase() == "n" {
            println!("\nThank you. Goodbye!");
            break;
        }
    }
}
