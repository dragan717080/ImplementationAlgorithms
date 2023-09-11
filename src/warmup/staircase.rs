use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n:i32 = line.trim().parse().unwrap_or_else(|_| panic!("Failed to parse integer"));

    for i in 1..n + 1 {
        let res: String = format!("{}{}", (0..n - i).map(|_| { " " }).collect::<String>(), (0..i).map(|_| { "#" }).collect::<String>());
        println!("{}", res);
    }
}
