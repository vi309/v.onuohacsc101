use std::io::Write;

fn main() {
    let output1 = vec!["Lager  ","33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let output2 = vec!["Stout  ","Legend","Turbo King","Williams"];
    let output3 = vec!["Non-Alcoholic  ","Maltina","Amstel Malta","Malta Gold","Fayrouz"];

    let mut file = std::fs::File::create("store.txt").unwrap();
    let title = format!("{:<15}{:<15}{:<15}\n", output1[0], output2[0], output3[0]);
    file.write_all(title.as_bytes()).unwrap();

    let max_len = output1.len().max(output2.len()).max(output3.len());
    for i in 1..max_len {
        let column1 = output1.get(i).unwrap_or(&"");
        let column2 = output2.get(i).unwrap_or(&"");
        let column3 = output3.get(i).unwrap_or(&"");
        let row = format!("{:<15}{:<15}{:<15}\n", column1, column2, column3);
        file.write_all(row.as_bytes()).unwrap(); 
    }
}
