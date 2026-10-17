use std::io;

fn main() {
    // Vector of tuples: (name, years of experience)
    let candidates = vec![
        ("Chinedu", 5),
        ("Aisha", 8),
        ("Tunde", 3),
        ("Ngozi", 12),
        ("Emeka", 7),
        ("Fatima", 10),
    ];

    println!("===== EY Nigeria Interview =====");
    println!("List of Candidates:\n");

    // Display all candidates
    for (name, years) in &candidates {
        println!("Name: {:<10} | Experience: {} years", name, years);
    }

    // Find the person with the highest experience
    let mut highest_name = "";
    let mut highest_years = 0;

    for (name, years) in &candidates {
        if *years > highest_years {
            highest_years = *years;
            highest_name = name;
        }
    }

    // Display the result
    println!("\n---------- RESULT ----------");
    println!("The candidate with the highest experience is:");
    println!("Name       : {}", highest_name);
    println!("Experience : {} years", highest_years);
    println!("----------------------------");
}