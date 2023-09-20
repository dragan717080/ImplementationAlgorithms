use std::{io, collections::HashMap};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: u32 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let mut freqs: HashMap<u32, u32> = HashMap::new();

    for d in &a {
        *freqs.entry(*d).or_insert(0) += 1;
    }
    
    println!("{}", a.len() as u32 - *freqs.values().cloned().collect::<Vec<u32>>().iter().filter(|&&d| d > 1).collect::<Vec<&u32>>().iter().max().unwrap_or(&&1_u32));
}
