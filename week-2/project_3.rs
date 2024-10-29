fn main() {
    let p:f64 = 510000.0;
    let r:f64 = 5.0;
    let _t:f64 = 3.0;
    let a:f64 = 1.0 - (r/ 100.0);
    let c:f64 = a.powf(3.0);
    let z:f64 = p * c;
    println!("depreciation is {}", z);
}