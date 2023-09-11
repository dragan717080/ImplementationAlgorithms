use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let q: i32 = line.trim().parse().unwrap();
    
    for _ in 0..q {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
        let [mut x, mut y, mouse] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

        x = (x - mouse).abs();
        y = (y - mouse).abs();

        if x > y {
            println!("{}", "Cat B");
        }
        else if x < y {
            println!("{}", "Cat A");
        }
        else {
            println!("{}", "Mouse C");
        }
    }
    
}
