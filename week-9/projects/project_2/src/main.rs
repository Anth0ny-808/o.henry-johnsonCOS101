use std::io::Write;

fn main() {
    let heading = "PAU Student Management Information System\n";
    let line = "==================================================\n\n";

    let header = "Student Name          Matric Number     Department      Level\n";
    let divider = "--------------------------------------------------\n";

    let student1 = "Oluchi Mordi          ACC10211111       Accounting      300\n";
    let student2 = "Adams Aliyu           ECO10110101       Economics      100\n";
    let student3 = "Shania Bolade         CSC10228828       Computer       200\n";
    let student4 = "Adekunle Gold         EEE11020202       Electrical     200\n";
    let student5 = "Blanca Edemoh         MEE10202001       Mechanical     100\n";

    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed");

    file.write_all(heading.as_bytes()).expect("write failed");
    file.write_all(line.as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(divider.as_bytes()).expect("write failed");
    file.write_all(student1.as_bytes()).expect("write failed");
    file.write_all(student2.as_bytes()).expect("write failed");
    file.write_all(student3.as_bytes()).expect("write failed");
    file.write_all(student4.as_bytes()).expect("write failed");
    file.write_all(student5.as_bytes()).expect("write failed");

    println!("\nData written to file.");
}
