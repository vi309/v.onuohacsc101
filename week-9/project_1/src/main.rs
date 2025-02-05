struct laptop {
    brand: String,
    quantity: u32,
    price: u32,
}
impl laptop {
    fn total_cost(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    println!("Welcome to  Ogbeifuna Electronics");
let hp = laptop {
    brand:String::from("HP"),
    quantity:3,
    price:650000,
};

let ibm = laptop {
    brand:String::from("IBM"),
    quantity:3,
    price:755000,
};

let toshiba = laptop {
    brand:String::from("TOSHIBA"),
    quantity: 3,
    price: 550000,
};

let dell = laptop {
    brand:String::from("DELL"),
    quantity: 3,
    price: 850000,
};
let total_cost = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();

println!("The total cost of your purchase is : {}",total_cost);
}
