use std::io;

// Avoid this problem, simple to solve but very hard to understand
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    for i in 0..numbers.len() {
        let j = i + 1;
        let mut first_index = numbers.iter().position(|&x| x == (j as i32)).unwrap();
        first_index += 1;
        let mut second_index = numbers.iter().position(|&x| x == (first_index as i32)).unwrap();
        second_index += 1;
        println!("{}", second_index);
    }
}
