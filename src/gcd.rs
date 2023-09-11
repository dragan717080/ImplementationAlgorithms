fn main() {
    println!("Hi");
    let result = gcd(245, 98);
    println!("{}", result);
}

fn gcd(p: i32, q: i32) -> i32 {
    if q == 0 {
        return p;
    }

    gcd(q, p % q)
}
