 use std::io
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Kindly Enter the coefficient of a");
    io::stdin.read_line(&mut input1).expect("Not an integer");
    let a:i32 = input1.trim().parse().expect("Not an integer");

    println!("Kindly enter the coefficient of b");
    io::stdin.read_line(&mut input2).expect("Not an integer");
    let b:i32 = input2.trim().parse().expect("Not an integer");

    println!("Kindly enter the coefficient of c");
    io::stdin.read_line(&mut input3).expect("Not an integer");
    let c:i32 = input3.trim().parse().expect("Not an integer");


    let discriminant = b * b - 4 * a * c
}
