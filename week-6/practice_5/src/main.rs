fn main() {
    let num:i32 = 5;
    mutate(num);
    println!("The value of no is:{}",num);
}
fn mutate(mut help: i32) {
    help = help*0;
    println!("help value is :{}",help);
}