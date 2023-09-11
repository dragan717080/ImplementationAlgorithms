use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap_or_else(|_| panic!("Failed to parse integer"));

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let [d, m] = [arr.next().unwrap(), arr.next().unwrap()];

    let mut result = 0;
    for i in 0..numbers.len() {
        let end = i + m as usize;

        if end <= numbers.len() {
            let slice = &numbers[i..end];
            
            if slice.iter().sum::<i32>() == d {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
