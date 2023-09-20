use std::{io, collections::HashMap};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    println!("{}", min_distance(a));
}

// Minimum distance between equal elements
fn min_distance(a: Vec<i32>) -> i32 {
    let mut min_dist = a.len() as i32;
    let mut distances_map: HashMap<i32, i32> = HashMap::new();

    for (i, &val) in a.iter().enumerate() {
        if let Some(prev_index) = distances_map.get(&val) {
            min_dist = min_dist.min((i as i32 - *prev_index).abs());
        }
        distances_map.insert(val, i as i32);
    }

    if min_dist == a.len() as i32 {
        -1
    } else {
        min_dist
    }
}