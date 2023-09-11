use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<usize>().unwrap());
    let [n, m] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; m]; n];

    for i in 0..n {  
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let chars: Vec<u32> = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
        for j in 0..chars.len() {
            matrix[i][j] = chars[j];
        } 
    }

    println!("{}", find_topics(matrix).iter().map(|d| d.to_string()).collect::<Vec<String>>().join("\n"));
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

fn find_topics(matrix: Vec<Vec<u32>>) -> [u32; 2] {
    let mut all_topics: Vec<u32> = Vec::new();

    for i in 0..matrix.len() - 1 {
        for j in (i + 1)..matrix.len() {
            let total_between_teams: u32 = 0;
            let topics: Vec<Vec<u32>> = transpose(vec![matrix[i].clone(), matrix[j].clone()]);

            let sum_for_two_teams: usize = topics
            .iter()
            .filter(|topic| topic[0] + topic[1] >= 1)
            .count();

            all_topics.push(sum_for_two_teams as u32);
        }
    }

    let max: u32 = *all_topics.iter().max().unwrap();
    [max, all_topics.iter().filter(|&&d| d == max).count() as u32]
}
