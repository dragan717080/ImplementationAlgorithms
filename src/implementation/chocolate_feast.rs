use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();
    
    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
        let [n, c, m] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
        println!("{}", chocolate_feast(n, c, m));
    }
}

fn chocolate_feast(n: i32, c: i32, m: i32) -> i32 {
    let b0 = n / c;
    let total_bars = b0 as f64 / (1.0 - 1.0 / m as f64);
    (total_bars - 1.0).ceil() as i32
}
