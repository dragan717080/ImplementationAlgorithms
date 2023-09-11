use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i64> = line.split_whitespace().map(|c| c.parse::<i64>().unwrap_or_else(|_| panic!("Failed to parse integer"))).collect();

    let mut sums: Vec<i64> = Vec::new();

    for i in 0..numbers.len() {
        let mut arr: Vec<i64> = numbers.iter().take(i).cloned().collect();
        let to_take_from_rev = numbers.len() - i - 1;
        let mut reversed: Vec<i64> = numbers.iter().cloned().rev().collect();
        reversed = reversed.iter().take(to_take_from_rev).cloned().collect();
        arr.extend(reversed);
        sums.push(arr.iter().sum());
    }

    println!("{} {}", sums.iter().min().unwrap(), sums.iter().max().unwrap());
}
