use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [n, k] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [r_q, c_q] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut obstacles: Vec<Vec<i32>> = Vec::new();
    
    for _ in 0..k {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        obstacles.push(line.split_whitespace().map(|c| c.parse().unwrap()).collect());
    }

    println!("{}", attack(n, r_q, c_q, obstacles));
}

fn attack(n: i32, r_q: i32, c_q: i32, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut b = vec![
        vec![0, n + 1],
        vec![0, n + 1],
        vec![std::cmp::max(c_q + r_q - n - 1, 0), std::cmp::min(r_q + c_q, n + 1)],
        vec![std::cmp::max(0, c_q - r_q), n + 1 + std::cmp::min(0, c_q - r_q)],
    ];

    for o in obstacles.iter() {
        if o[1] == c_q {
            if o[0] < r_q && o[0] > b[1][0] {
                b[1][0] = o[0];
            }
            if o[0] > r_q && o[0] < b[1][1] {
                b[1][1] = o[0];
            }
            continue;
        }

        let i: usize;
        if o[0] == r_q {
            i = 0;
        } 
        else if o[0] + o[1] == r_q + c_q {
            i = 2;
        } 
        else if o[0] - o[1] == r_q - c_q {
            i = 3;
        } 
        else { continue; }

        if o[1] < c_q && o[1] > b[i][0] {
            b[i][0] = o[1];
        }
        if o[1] > c_q && o[1] < b[i][1] {
            b[i][1] = o[1];
        }
    }

    b.iter().fold(-8, |acc, o| acc + o[1] - o[0])
}
