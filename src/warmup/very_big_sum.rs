use std::io::{self};

fn big_sum(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n:i32 = line.trim().parse().expect("Failed to parse integer");

    line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    if n > arr.len() as i32 {
        panic!("Array must be at least {} long", n);
    }

    if n < (arr.len() as i32) {
        arr.truncate(n as usize);
    }

    let res = big_sum(&arr);
    println!("{}", res);
}
