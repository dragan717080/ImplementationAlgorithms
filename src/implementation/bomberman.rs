use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [r, _, n] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut grid: Vec<String> = Vec::new();
    
    for _ in 0..r {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        grid.push(line.trim().to_owned());
    }

    for row in bomberman(n, grid) {
        println!("{}", row);
    }
}

fn bomberman(n: i32, grid: Vec<String>) -> Vec<String> {
    if n == 1 {
        return grid;
    }

    const BOMB: char = 'O';
    const EMPTY: char = '.';

    if n % 2 == 0 {
        return grid.iter().map(|line| line.replace(EMPTY, &BOMB.to_string())).collect();
    }

    fn get_final_state(p_grid: &Vec<String>) -> Vec<String> {
        let mut final_state = Vec::new();
        for (i, line) in p_grid.iter().enumerate() {
            let mut new_line = String::new();
            for (j, cell) in line.chars().enumerate() {
                match cell {
                    BOMB => new_line.push(EMPTY),
                    _ if j > 0 && p_grid[i].chars().nth(j - 1) == Some(BOMB)
                        || i > 0 && p_grid[i - 1].chars().nth(j) == Some(BOMB)
                        || j < line.len() - 1 && p_grid[i].chars().nth(j + 1) == Some(BOMB)
                        || i < p_grid.len() - 1 && p_grid[i + 1].chars().nth(j) == Some(BOMB)
                    => new_line.push(EMPTY),
                    _ => new_line.push(BOMB),
                }
            }
            final_state.push(new_line);
        }
        final_state
    }

    let final_state = get_final_state(&grid);
    if n % 4 != 1 {
        return final_state;
    }

    get_final_state(&final_state)
}