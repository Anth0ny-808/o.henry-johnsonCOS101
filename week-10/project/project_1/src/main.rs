struct Laptop {
    brand: String,
    price: f64,
}

impl Laptop {

    fn cost(&self, quantity: i32) -> f64 {
        self.price * quantity as f64
    }
}

fn main() {
    
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000.0,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000.0,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000.0,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000.0,
    };

    let quantity = 3;

    
    let hp_cost = hp.cost(quantity);
    let ibm_cost = ibm.cost(quantity);
    let toshiba_cost = toshiba.cost(quantity);
    let dell_cost = dell.cost(quantity);

    
    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    
    println!("      Laptop Purchase Cost      \n");
    println!("{}  : 3 × ₦{:.0} = ₦{:.0}", hp.brand, hp.price, hp_cost);
    println!("{} : 3 × ₦{:.0} = ₦{:.0}", ibm.brand, ibm.price, ibm_cost);
    println!("{}: 3 × ₦{:.0} = ₦{:.0}", toshiba.brand, toshiba.price, toshiba_cost);
    println!("{}: 3 × ₦{:.0} = ₦{:.0}", dell.brand, dell.price, dell_cost);

    println!("\nTotal Cost = ₦{:.0}", total);
}