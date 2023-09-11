use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: u8 = line.trim().parse().unwrap();

    for _ in 0..t {     
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<u32>().unwrap());
        let [n, m, s] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
        
        let res = find_result_arr(n, m, s);
        println!("{}", res);
    }
}

fn find_result_arr(n: u32, m: u32, s: u32) -> u32 {
    (s + m - 2) % n + 1
}
