use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        
        matrix.push(line.trim().chars().into_iter().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    for row in find_map(matrix) {
        println!("{}", row);
    }
}

fn find_map(matrix: Vec<Vec<i32>>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let cols = matrix[0].len();

    if cols < 3 {
        res = matrix.iter().map(|row| row.iter().map(|&num| num.to_string()).collect::<String>()).collect();
        return res;
    }

    for i in 0..cols {
        let mut row_str = String::new();

        for j in 0..cols {
            if !(1..(cols - 1)).contains(&i) || !(1..(cols - 1)).contains(&j) {
                row_str.extend(matrix[i][j].to_string().chars());
            }
            else {
                row_str.extend(if !has_cavity(&matrix, i, j) { matrix[i][j].to_string() } else { "X".to_string() }.chars());
            }
        }
        res.push(row_str);
    }

    res
}

fn has_cavity(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let adjacent_cells: [i32; 4] = [matrix[row - 1][col], matrix[row][col - 1], matrix[row][col + 1], matrix[row + 1][col]];

    for adjacent_cell in adjacent_cells {
        if adjacent_cell >= matrix[row][col] {
            return false;
        }
    }

    true
}
