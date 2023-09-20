use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [x1, v1, x2, v2] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    
    println!("{}", jumps(x1, v1, x2, v2));
}

fn jumps(mut x1: i32, v1: i32, mut x2: i32, v2: i32) -> &'static str {
    if v2 >= v1 { 
        return "NO"; 
    }
    while x2 > x1 {
        x1 += v1;
        x2 += v2;
    }
    if x1 == x2 { "YES" } else { "NO" }
}
