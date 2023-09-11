use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<u32>().unwrap());
    let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut bill: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let b: u32 = line.trim().parse().unwrap();

    bill.remove(k as usize);

    let a = bill.iter().sum::<u32>() / 2;
    let res: String = if b > a { (b - a).to_string() } else { "Bon Appetit".to_string() };
    println!("{}", res);
}
