use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [h, _] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..h {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        matrix.push(line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect());
    }

    println!("{}", surface(&matrix));
}

fn surface(a: &[Vec<i32>]) -> i32 {
    let rows = a.len();
    let cols = a[0].len();
    let mut total = ((rows * cols) * 2) as i32;

    for i in 0..rows {
        for j in 0..cols {
            let value = a[i][j];
            let mut max = value * 4;

            if i > 0 {
                max -= compare(value, a[i - 1][j]);
            }
            if j > 0 {
                max -= compare(value, a[i][j - 1]);
            }
            if i + 1 < rows {
                max -= compare(value, a[i + 1][j]);
            }
            if j + 1 < cols {
                max -= compare(value, a[i][j + 1]);
            }

            total += max;
        }
    }

    total
}

fn compare(value1: i32, value2: i32) -> i32 {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}
