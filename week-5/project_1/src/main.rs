use std::io;
fn main() {
    let _p = 3200;
    let _f = 3000;
    let _a = 2500;
    let mut _total_price = 0;

    println!("P= Poundo Yam/Edinkaiko Soup -N3200 ");
    println!("F= Fried Rice and chicken -N3000");
    println!("A= Amala and Ewedu Soup -2500");
    
    let mut menu_option = String::new();
    let mut quantity = String::new();

    println!("Please select a menu option");
    
    io::stdin().read_line(&mut menu_option).expect("failed to read input");
    
    println!("Please select the quantity");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let _quantity:i32 = quantity.trim().parse().expect("Invalid quantity");

    println!("{}",menu_option);
        
    if menu_option.trim() == "P" || menu_option.trim() == "p" {
        _total_price = _p * _quantity;
        println!("Total price for order is {}", _total_price);
    }
    else if menu_option.trim() == "F" || menu_option.trim() == "f" {
        _total_price = _f * _quantity;
        println!("Total price for order is {}", _total_price);
    }
    else if menu_option.trim() == "A" || menu_option.trim() == "a" {
        _total_price = _a * _quantity;
        println!("Total price for order is {}", _total_price);
    }
    else{
        println!("Invalid menu option");
    }

    
}
