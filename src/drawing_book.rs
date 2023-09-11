use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let n: i32 = line.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let p: i32 = line.trim().parse().unwrap();

    let count = page_count(n, p);
    println!("{}", count);
}

fn page_count(n: i32, p: i32) -> i32 {
    std::cmp::min(p / 2, (n - p) / 2)
}
