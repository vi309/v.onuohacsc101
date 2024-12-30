use std::io;
use std::io::Write;

fn main() {
    let mut department: Vec<String> = Vec::new();
    let mut matricno: Vec<String> = Vec::new();
    let mut level: Vec<String> = Vec::new();
    let mut name: Vec<String> = Vec::new();

    let mut file = std::fs::File::create("sibas.txt").unwrap();
    file.write_all("PAU SMIS\n".as_bytes()).unwrap();
    let title = format!("{:<40}{:<40}{:<40}{:<40}\n","Name", "Matric","Department","Level");
    file.write_all(title.as_bytes()).unwrap();

    loop{
        let mut input = String::new();
        println!("Enter your name or type 'end' when done");
        io::stdin().read_line(&mut input).unwrap();
        let identity: String = input.trim().to_string();
        if &identity == "end" {
            break;
        }
        name.push(identity.clone());
    }
}
