use std::io;

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    // read variables
    io::stdin().read_line(&mut line1).unwrap();
    io::stdin().read_line(&mut line2).unwrap();

    // parse integers
    let a : i32 = line1.trim().parse().unwrap();
    let b : i32 = line2.trim().parse().unwrap();

    println!("{}", a + b);
}