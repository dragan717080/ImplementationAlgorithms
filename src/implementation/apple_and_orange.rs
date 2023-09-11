use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let [s, t] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let [a, b] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let [_, _] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let apples: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let oranges: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let apple_indexes: Vec<i32> = apples.iter().map(|n| n + a).collect();
    let apples_range: usize = apple_indexes.iter().filter(|n| n >= &&s && n <= &&t).count();

    println!("{}", apples_range);

    let orange_indexes: Vec<i32> = oranges.iter().map(|n| n + b).collect();
    let oranges_range: usize = orange_indexes.iter().filter(|n| n >= &&s && n <= &&t).count();

    println!("{}", oranges_range);
}
