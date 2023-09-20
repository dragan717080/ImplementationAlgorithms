use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let n: usize = line.trim().parse().unwrap();
        println!("{}", find_tree(n));
    }
}

fn find_tree(n: usize) -> i32 {
    let mut res = 0;
    for i in 0..(n + 1) {
        res += if i % 2 == 0 { 1 } else { res };
    }

    res
}
