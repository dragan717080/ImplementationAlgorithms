use std::io::{self, Read, BufRead};
mod utils;

use utils::read_numbers_from_input;

use std::convert::TryInto;

fn main() {
    let [n, m]: [i32; 2] = (&read_numbers_from_input()[0..2]).try_into().unwrap();

    let all_topics = input_to_number_list(n);

    let result = find_topics_combinations(&all_topics);
    for item in result {
        println!("{}", item);
    }
}

fn input_to_number_list(n: i32) -> Vec<Vec<i32>> {
    let arr: Vec<Vec<i32>> = (0..n).map(|_| str_to_numbers_arr()).collect();

    fn str_to_numbers_arr() -> Vec<i32> {
        let stdin = io::stdin();
        let s = stdin.lock().lines().next().unwrap().unwrap();
        let numbers: Vec<i32> = s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect();
        numbers
    }

    arr
}

fn transpose<T: Clone + Default>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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

fn find_topics_combinations(all_topics: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut known_topics: Vec<i32> = vec![];

    for team1 in 0..(all_topics.len() - 1) {
        for team2 in (team1 + 1)..all_topics.len() {
            let mut topics: Vec<i32> = vec![];
            for i in 0..all_topics[0].len() {
                topics.push(if all_topics[team1][i] != 0 || all_topics[team2][i] != 0 { 1 } else { 0 });
            }
            topics = topics.iter().filter(|&&n| n == 1).cloned().collect();
            
            known_topics.push(topics.len() as i32);
        }
    }

    let max_known_topics = known_topics.iter().max().unwrap();

    let result: Vec<i32> = vec![*max_known_topics, known_topics.iter().filter(|&&topic| topic == *max_known_topics).count() as i32];
    
    result
}
