use std::io;

fn main() {
    let mut line = String::new();
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..3 {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        matrix.push(line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect());
    }

    println!("{}", square(&matrix));
}

fn square(matrix: &[Vec<i32>]) -> i32 {

/* Remember one magic square, then derive 3 more squares by rotating it in clockwise direction.
    Then transpose any magic square, that'll be 5th magic square.
    Now to obtain 3 more magic squares, rotate the 5th magic square 3 more times.
    Now traverse in each magic square and calculate the cell difference
    Store the minimum difference */

let magic_squares: Vec<Vec<Vec<i32>>> = vec![
        vec![
            vec![8, 1, 6],
            vec![3, 5, 7],
            vec![4, 9, 2],
        ],
        vec![
            vec![6, 1, 8],
            vec![7, 5, 3],
            vec![2, 9, 4],
        ],
        vec![
            vec![4, 9, 2],
            vec![3, 5, 7],
            vec![8, 1, 6],
        ],
        vec![
            vec![2, 9, 4],
            vec![7, 5, 3],
            vec![6, 1, 8],
        ],
        vec![
            vec![8, 3, 4],
            vec![1, 5, 9],
            vec![6, 7, 2],
        ],
        vec![
            vec![4, 3, 8],
            vec![9, 5, 1],
            vec![2, 7, 6],
        ],
        vec![
            vec![6, 7, 2],
            vec![1, 5, 9],
            vec![8, 3, 4],
        ],
        vec![
            vec![2, 7, 6],
            vec![9, 5, 1],
            vec![4, 3, 8],
        ],
    ];

    let mut min_cost = i32::MAX;

    for magic_square in &magic_squares {
        let mut cost = 0;
        for i in 0..3 {
            for j in 0..3 {
                cost += (matrix[i][j] - magic_square[i][j]).abs();
            }
        }
        min_cost = min_cost.min(cost);
    }

    min_cost
}
