use std::io::Write;
fn main() {
    let com = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye", 
    ];
    let min = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let geo = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
    let mut file = std::fs::File::create("combiner.txt").unwrap();
    let columns = format!("{:<5} {:<30} {:<20} {:<15}\n", "S/N","Name of Commissioner","Ministry", "Geopolitical zone");
    file.write_all(columns.as_bytes()).unwrap();

    for i in 0..com.len() {
        let row = format!(
            "{:<5} {:<30} {:<20} {:<15}\n",
            i + 1,
            com[i],
            min[i],
            geo[i] 
        );
        file.write_all(row.as_bytes()).unwrap();
    }
}
