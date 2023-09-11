use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let n: i32 = line.trim().parse().unwrap();
        
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let a: i32 = line.trim().parse().unwrap();

        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let b: i32 = line.trim().parse().unwrap(); 
    
        let res = stones(n, a, b);
        println!("{}", res.iter().map(|&d| d.to_string()).collect::<Vec<String>>().join(" "));
    }    
}

fn stones(n: i32, a: i32, b: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for x in 0..n {
        res.push(x * a + (n - 1 - x) * b)
    }

    let set: HashSet<i32> = res.clone().into_iter().collect();

    res = set.into_iter().collect();
    res.sort();

    res
}
