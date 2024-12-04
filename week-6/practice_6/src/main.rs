fn main() {
    let mut num:i32 = 5;
    help(&mut num);
    println!("The value of num is: {}",num);
}

fn help(param:&mut i32) {
    *param = *param*0;
    println!("param value is : {}",param);
}
