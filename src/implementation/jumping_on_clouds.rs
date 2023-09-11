use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn jumping_on_clouds(c: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut index = 0;

    while index < c.len() - 1 {
        if index + 2 < c.len() && c[index + 2] == 1 {
            index += 1;
        } else {
            index += 2;
        }
        result += 1;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let c: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumping_on_clouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
