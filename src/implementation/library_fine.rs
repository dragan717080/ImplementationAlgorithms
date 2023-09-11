use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [d1, m1, y1] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [d2, m2, y2] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

    println!("{}", calculate(d1, m1, y1, d2, m2, y2));
}

fn calculate(d1: i32, m1: i32, y1: i32, d2: i32, m2: i32, y2: i32) -> i32 {
    if m1 == m2 && y1 == y2 && d1 >= d2 {
        return 15*(d1 - d2);
    }
    else if m1 > m2 && y1 == y2 {
        return 500*(m1-m2);
    }
    else if y1 > y2 {
        return 10000;
    }
    else {
        return 0;
    }
}
