use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n:i32 = line.trim().parse().unwrap_or_else(|_| panic!("Failed to parse integer"));

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let (positive_numbers, negative_numbers, zeros) = &numbers.into_iter().fold(
        (0, 0, 0), |(mut pos, mut neg, mut zeros), num| {
            if num > 0 {
                pos += 1;
            } else if num < 0 {
                neg += 1;
            } else {
                zeros += 1;
            }
            (pos, neg, zeros)
        },
    );
    for item in [positive_numbers, negative_numbers, zeros] {
        println!("{:.6?}", *item as f64 / n as f64);
    }
}
