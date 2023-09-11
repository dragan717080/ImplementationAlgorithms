use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let p: u32 = line.trim().parse().unwrap();

    let turns_from_start: u32 = p / 2;
    let turns_from_end: u32 = n / 2 - p / 2;
    println!("{}", vec![turns_from_start, turns_from_end].iter().min().unwrap());
}
