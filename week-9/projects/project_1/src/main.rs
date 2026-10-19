use std::io::Write;

fn main() {
    let heading = "Nigerian Breweries - Drink Categories\n";
   

    let lager = "LAGER:\n- 33 Export\n- Desperados\n- Goldberg\n- Gulder\n- Heineken\n- Star\n\n";
    let stout = "STOUT:\n- Legend\n- Turbo King\n- Williams\n\n";
    let non_alcoholic = "NON-ALCOHOLIC:\n- Maltina\n- Amstel Malta\n- Malta Gold\n- Fayrouz\n";

    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");

    file.write_all(heading.as_bytes()).expect("write failed");
    
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("\nData written to file.");
}