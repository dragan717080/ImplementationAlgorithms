use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
    
    let max = numbers.iter().max().unwrap().clone();
    let res: i32 = if max > k { max - k } else { 0 };
    println!("{}", res); 
}
