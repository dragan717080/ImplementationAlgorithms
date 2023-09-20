use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let s = line.trim().to_owned();
    println!("{}", find_encrypted(&s));
}

fn find_encrypted(s: &str) -> String {
    let s = s.replace(" ", "");

    if s.len() == 1 {
        return s.to_owned();
    }

    let rt = (s.len() as f64).sqrt();
    let cols: i32 = rt.ceil() as i32;
    let rows: i32 = if rt.floor() as i32 * cols >= s.len() as i32 { rt.floor() as i32 } else { cols };

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for i in 0..rows {
        let mut row_str: Vec<char> = Vec::new();

        for j in 0..cols {
            if let Some(c) = s.chars().nth((i * cols + j) as usize) {
                row_str.push(c);
            }
            else {
                row_str.push(' ')
            }
        }
        matrix.push(row_str);
    }

    let mut res: String = String::new();
    let transposed = transpose(matrix);

    for (i, row) in transposed.iter().enumerate() {
        let mut row_str: String = row.iter().filter(|&c| c != &' ').collect();
        if i != transposed.len() {
            row_str.push(' ');
        }
        res.extend(row_str.chars());
    }

    res
}

fn transpose<T: Clone + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut res: Vec<Vec<T>> = vec![vec![Default::default(); rows]; cols];

    for i in 0..cols {
        for j in 0..rows {
            res[i][j] = matrix[j][i].clone();
        }
    }

    res
}
