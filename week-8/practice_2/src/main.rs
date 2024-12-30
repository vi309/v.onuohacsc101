use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = Strings::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents)
}
