use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    for _ in 0..n {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();

        let number: i32 = line.trim().parse().unwrap();

        let digits = find_digits(number);
        let factors = find_factors(number);

        let res: usize = digits.iter().filter(|d|factors.contains(&d)).count();

        println!("{}", res);
    }
}

fn find_factors(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as i32;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
    }

    factors.sort();

    factors
}

fn find_digits(p: i32) -> Vec<i32> {
    p.to_string().chars().into_iter().map(|c| c.to_digit(10).unwrap()).map(|d| d as i32).collect()
}
