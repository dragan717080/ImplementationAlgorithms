use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let result: String = fair_rations(&mut numbers);
    println!("{}", result);
}

fn fair_rations(b: &mut Vec<u32>) -> String {
    let mut res = 0;

    for i in 0..b.len() - 1 {
        b[i + 1] += b[i] % 2;
        res += b[i] % 2;
    }

    if b[b.len() - 1] % 2 == 0 {
        (res * 2).to_string()
    } else {
        "NO".to_string()
    }
}
