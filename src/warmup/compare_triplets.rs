use std::io;

fn main() {

    // Read the two lines of input
    // Parse the first line into integers and collect them into a Vec
    let triplets: Vec<Vec<i32>> = (0..2).map(|_| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|s| s.trim().parse().expect("Failed to parse integer"))
            .collect()
    }).collect();

    let res = compare(transpose(triplets)).iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", res);
}

fn transpose<T: Clone + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![Default::default(); rows]; cols];

    for i in 0..cols {
        for j in 0..rows {
            result[i][j] = matrix[j][i].clone();
        }
    }

    result
}

fn compare(matrix: Vec<Vec<i32>>) -> Vec<i32> {

    let [mut a, mut b] = [0, 0];

    for arr in matrix {
        if arr[0] > arr[1] {
            a += 1;
        }
        else if arr[0] < arr[1] {
            b += 1;
        }
    }

    vec![a, b]
}
