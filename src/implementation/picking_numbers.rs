use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: u32 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let max_count = a.iter().map(|&k| a.iter().filter(|&&p| p == k || p == k + 1).count()).max().unwrap();

    println!("{}", max_count);
}
