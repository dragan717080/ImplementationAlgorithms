use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: u16 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let res = cut_sticks(&a).clone();
    println!("{}", res.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("\n"));
}

fn cut_sticks(a: &[i32]) -> Vec<i32> {
    let mut a = a.to_vec();
    let mut res: Vec<i32> = vec![a.len() as i32];

    while a.len() > 1 {
        let x = *a.iter().min().unwrap();
        let count = a.iter().filter(|&&d| d == x).count();
        let y = a.len() - count;

        if y > 0 {
            res.push(y as i32);
        }
        a = a.into_iter().filter(|&d| d != x).collect();
    }

    res
}
