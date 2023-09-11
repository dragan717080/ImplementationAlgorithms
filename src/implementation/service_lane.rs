use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, t] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let w: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    for _ in 0..t {  
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<usize>().unwrap());
        let [i, j] = [numbers.next().unwrap(), numbers.next().unwrap()];
        
        let res = &w[i..=j].iter().min().unwrap();
        println!("{}", res);
    }
}
