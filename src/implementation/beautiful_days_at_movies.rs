use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [i, j, k] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    println!("{:?}", (i..=j).filter(|&d| is_beautiful(d, k)).collect::<Vec<i32>>().len());
}

fn is_beautiful(d: i32, k: i32) -> bool {
    let n: i32 = d.to_string().chars().rev().collect::<Vec<char>>().into_iter().collect::<String>().parse().unwrap();
    (d - n).abs() % k == 0
}
