use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let q: usize = line.trim().parse().unwrap();
    
    for _ in 0..q {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let n: i128 = line.trim().parse().unwrap();
        
        let mut matrix: Vec<Vec<i128>> = Vec::new();
        for _ in 0..n {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            matrix.push(line.split_whitespace().map(|c| c.parse::<i128>().unwrap()).collect());
        }
        println!("{}", can_organize(&matrix));
    }
}

fn can_organize(matrix: &[Vec<i128>]) -> &'static str {
    let transposed = transpose(matrix.to_vec());
    let mut row_sums: Vec<i128> = matrix.iter().map(|row| row.iter().sum()).collect();
    let mut col_sums: Vec<i128> = transposed.iter().map(|col| col.iter().sum()).collect();
    row_sums.sort();
    col_sums.sort();

    if row_sums == col_sums { "Possible" } else { "Impossible" }
}

fn transpose<T: Default + Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();

    let mut res: Vec<Vec<T>> = vec![vec![Default::default(); rows]; cols];

    for i in 0..cols {
        for j in 0..rows {
            res[i][j] = matrix[j][i].clone();
        }
    }

    res
}
