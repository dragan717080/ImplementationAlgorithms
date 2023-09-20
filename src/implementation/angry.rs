use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {  
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
        let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];
        
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let a: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

        println!("{}", cancel(k, &a));
    }
}

fn cancel(k: i32, a: &Vec<i32>) -> &'static str {
   if a.iter().filter(|&d| d <= &0).count() >= k as usize { "NO" } else { "YES" }
}
/* 4 3
-1 -3 4 2
4 2
0 -1 2 1 */