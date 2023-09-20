use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [p, d, m , mut s] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

    println!("{}", find_n(p, d, m, &mut s));
}

fn find_n(p: i32, d: i32, m: i32, s: &mut i32) -> i32 {
    let mut res: i32 = 0;
    let mut current_cost = p.clone();

    while s >= &mut current_cost {
        *s -= current_cost;
        if current_cost -d > m {
            current_cost -= d;
        }
        else {
            current_cost = m;
        }
        res += 1;
    }

    res
}
