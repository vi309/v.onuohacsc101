use std::io;
fn main() {
    println!("WELCOME TO Treisten's CALCULATOR!");
    println!("Type 'T' for Area of a trapezium");
    println!("Type 'R' for Area of a rhombus");
    println!("Type 'P' for Area of a parallelogram");
    println!("Type 'C' for Area of a cube");
    println!("Type 'V' for volume of a cylinder");
    let mut input = String::new();
  let mut input_2 =  String::new(); 
  let mut diagonal_1 = String::new();
        let mut diagonal_2 = String::new();  
  println!("What would you like to calculate?");
    io::stdin().read_line(&mut input).expect("Invalid option");
    

    if input.trim() == "t" || input.trim() == "T" {
        println!("input height of trapezium");
        io::stdin().read_line(&mut input_2).expect("Invalid input");
        let ii:f64 = input_2.trim().parse().expect("Invalid input");
        
        println!("input base 1");
        let mut base_1 = String::new();
        io::stdin().read_line(&mut base_1).expect("invalid input");
        let b_1:f64 = base_1.trim().parse().expect("invalid input");
        println!("input base 2");
        let mut base_2 = String::new();
        io::stdin().read_line(&mut base_2).expect("invalid input");
        let b_2:f64 = base_2.trim().parse().expect("invalid input");
        let at = (ii/2.0) * (b_1 + b_2);
        println!("Area of trapezium is {}",at);
    }
    else if input.trim() == "R" || input.trim() == "r" {
        println!("enter diagonal 1");
        io::stdin().read_line(&mut diagonal_1).expect("invalid input");
        let d_1:f64 = diagonal_1.trim().parse().expect("invalid input");
        
        println!("enter diagonal 2");
        io::stdin().read_line(&mut diagonal_2).expect("invalid input");
        let d_2:f64 = diagonal_2.trim().parse().expect("invalid input");
        
        let ar = 0.5 * d_1 * d_2;
 
        println!("Area of rhombus is {}",ar);
    }
    else if input.trim() == "p" || input.trim() == "P" {
        let mut base = String::new();
        println!("enter base");
        io::stdin().read_line(&mut base).expect("invalid input");
        let b:f64 = base.trim().parse().expect("invalid input");
        let mut alt = String::new();

        println!("enter altitude");
        io::stdin().read_line(&mut alt).expect("invalid input");
        let a:f64 = alt.trim().parse().expect("invalid input");
        let ap = b * a;

        println!("Area of parallelogram is {}",ap);

    }
    else if input.trim() == "C" || input.trim() == "c" {
        let mut lof = String::new();
        println!("enter length of side");
        io::stdin().read_line(&mut lof).expect("invalid input");
        let l:f64 = lof.trim().parse().expect("invalid input");
        let ac = 6.0 * l * l;
        println!("Area of cube is {}",ac);
    }
    else if input.trim() == "v" || input.trim() == "V" {
        let mut roh = String::new();
        let mut hoe = String::new();
        println!("enter height");
        io::stdin().read_line(&mut hoe).expect("invalid input");
        let h:f64 = hoe.trim().parse().expect("invalid input");

        println!("enter radius of cylinder");
        io::stdin().read_line(&mut roh).expect("invalid input");
        let r:f64 = roh.trim().parse().expect("invalid input");

        let vc = 3.143 * r * r * h;
        println!("Volume of cylinder is {}",vc);
    } 
    
}
