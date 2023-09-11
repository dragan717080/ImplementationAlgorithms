use std::io;

fn main() {

}

pub fn read_numbers_from_input() -> Vec<i32> {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

    line.split_whitespace()
    .map(|s| s.trim().parse().expect("Failed to parse integer"))
    .collect()
}
