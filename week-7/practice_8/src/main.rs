fn main() {
    elt mut mountain_heights = ("Everest",8848,"Fishtail",6993);
    println!("Original tuple = {:?}",mountain_heights);

    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Charged tuple = {:?}", mountain_heights);
}
