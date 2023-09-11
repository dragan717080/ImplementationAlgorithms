use std::io;

use indexmap::IndexMap;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");
    let t: i32 = line.trim().parse().unwrap();
    let v = get_counter_at_time(t).unwrap();
    println!("{}", v);
}

fn get_counter_at_time(t: i32) -> Option<i32> {
    let mut timer = 3;
    let mut steps = 1;

    if t < timer {
        return Some(timer + 1 - t);
    }

    while timer < t {
        timer += 6 * steps;
        steps += 1;
    }

    let keys: Vec<i32> = (timer + 1 - 6 * (steps - 1)..=timer).collect();
    let mut values: Vec<i32> = (1..=6 * (steps - 1)).collect();
    values = values.iter().rev().cloned().collect();

    let mut map = IndexMap::new();

    for (k, v) in keys.iter().zip(values.iter()) {
        map.insert(k, v);
    }

    map.get(&t).cloned().copied()
}
