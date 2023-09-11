// Medium
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
        let [r1, _]= [numbers.next().unwrap(), numbers.next().unwrap()];
    
        let mut g: Vec<String> = Vec::new();
    
        for _ in 0..r1 {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            g.push(line.trim().to_string());
        }
    
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
        let [r, _]= [numbers.next().unwrap(), numbers.next().unwrap()];
    
        let mut p: Vec<String> = Vec::new();
    
        for _ in 0..r {
            line.clear();
            io::stdin().read_line(&mut line).unwrap();
            p.push(line.trim().to_string());
        }
    
        println!("{}", find_pattern(g, p));
    }

}

fn find_pattern(g: Vec<String>, p: Vec<String>) -> &'static str {
    for gy in 0..=(g.len() - p.len()) {
        for gx in 0..=(g[0].len() - p[0].len()) {
            let mut ok = true;
            for py in 0..p.len() {
                for px in 0..p[0].len() {
                    if g[gy + py][gx + px..gx + px + 1] != p[py][px..px + 1] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                return "YES";
            }
        }
    }
    "NO"
}
