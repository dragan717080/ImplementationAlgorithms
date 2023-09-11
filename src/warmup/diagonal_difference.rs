use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n:i32 = line.trim().parse().expect("Failed to parse integer");

    let matrix: Vec<Vec<i32>> = (0..n).map(|_| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect()
    }).collect();

    let mut sum_l:i32 = 0;
    let mut sum_r:i32 = 0;
    for i in 0..matrix.len() {
        sum_l += matrix[i][i];
        sum_r += matrix[i][matrix.len() -i - 1];
    }

    println!("{}", (sum_l - sum_r).abs());
}
