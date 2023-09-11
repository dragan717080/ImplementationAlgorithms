use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap_or_else(|_| panic!("Failed to parse integer"));

    line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

    assert_eq!(numbers.len(), n);

    let max = numbers.iter().max().unwrap();
    let count = numbers.iter().filter(|n| n == &max).count();
    println!("{}", count);
}
