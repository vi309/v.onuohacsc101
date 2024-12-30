use std::fs;
fn main() {
    fs::remove_file("data_txt").unwrap();
    pritnln!("file is removed");
}
