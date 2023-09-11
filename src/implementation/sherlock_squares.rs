use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let q: i32 = line.trim().parse().unwrap();
    
    for _ in 0..q {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<u32>().unwrap());
        let [a, b] = [numbers.next().unwrap(), numbers.next().unwrap()];

        // Slower performing version
        // let squares: Vec<u32> = (a..=b).filter(|d| (*d as f64).sqrt().to_string().find('.') == None).collect();
        // println!("{}", squares.len());

        // Faster performing version
        println!("{}", squares_in_range(a, b));
    }
}

fn squares_in_range(a: u32, b: u32) -> u32 {
    let res: u32 = (b as f64).sqrt().floor() as u32 + 1 - (a as f64).sqrt().ceil() as u32;
    
    res
}
