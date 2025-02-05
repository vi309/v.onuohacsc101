fn main() {
    let x = vec![100, 200,300];
    borrow_vector(&x);

    println!("Printing the value from main() x[0]={}",x[0]);
    println!("dancers are the best");
}

fn borrow_vector(z:&Vec<i32>){
    println!("dancers are the best");
    println!("Inside print_vector function {:?} \n",z);
    println!("singers can sing");
}