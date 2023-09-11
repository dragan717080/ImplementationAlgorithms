use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i64 = line.trim().parse().unwrap();
    
    println!("{}", find_count(t));
}

fn find_count(t: i64) -> i64 {
    if t < 4 {
        return 4 - t;
    }

    let [mut _lower, mut upper, mut k] = [3, 3, 1];

    while upper < t {
        upper += 6*k;
        if t > upper {
            _lower += 6*k;
        }
        k *= 2;
    }

    upper - t + 1
}
