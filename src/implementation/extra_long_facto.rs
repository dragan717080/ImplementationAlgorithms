use std::io;

use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: u64 = line.trim().parse().unwrap();

    let res = (1..=n).map(|i| BigUint::from(i)).fold(BigUint::one(), |acc, x| acc * x);

    println!("{}", res);
}
