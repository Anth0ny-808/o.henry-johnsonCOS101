use std::fs::File;
use std::io::Write;

fn main() {
   
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Creating the file
    let mut file = File::create("efcc_merged.txt").expect("Failed to create file");

    
    writeln!(file, "EFCC Merged File").expect("Failed to write");
    writeln!(file, "================\n").expect("Failed to write");

    
    for i in 0..5 {
        writeln!(
            file,
            "{} | {} | {}",
            commissioners[i], ministries[i], zones[i]
        )
        .expect("Failed to write");
    }

    println!("File created successfully!");
}