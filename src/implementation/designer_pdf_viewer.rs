use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let word = line.trim();

    let mut letter_values: HashMap<char, u32> = HashMap::new();

    for i in 0..numbers.len() {
        letter_values.insert(char::from_u32((i + 97) as u32).unwrap(), numbers[i]);
    }
    
    let keys_to_retain: Vec<char> = word.chars().into_iter().collect();

    for key in letter_values.clone().keys() {
        if !keys_to_retain.contains(key) {
            letter_values.remove(key);
        }
    }
    println!("{}", word.len() as u32 * letter_values.values().into_iter().max().unwrap());
}
