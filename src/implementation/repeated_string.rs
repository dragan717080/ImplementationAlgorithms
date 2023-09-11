use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let s = line.trim().to_owned();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let n: u64 = line.trim().parse().unwrap();
    
    let repetitions: u64 = n / s.len() as u64;

    let res: u64 = s.chars().filter(|&c| c == 'a').count() as u64 * repetitions + s[0..(n as usize % s.len())].chars().filter(|&c| c == 'a').count() as u64;
    println!("{}", res);
}
