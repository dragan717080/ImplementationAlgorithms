use std::{io, collections::{HashMap, HashSet}};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
    let numbers_set: HashSet<i32> = numbers.clone().into_iter().collect();

    let mut freqs: HashMap<i32, i32> = HashMap::new();

    for number in numbers_set {
        freqs.insert(number, 0);
    }

    for number in numbers.clone() {
        freqs.entry(number).and_modify(|v| *v += 1);
    }
    
    let max: i32 = *freqs.values().into_iter().max().unwrap();
    let result_elements: Vec<i32> = freqs.keys().filter(|&k| freqs[k] == max).map(|&k| k).collect();

    println!("{:?}", result_elements.iter().min().unwrap());
}
