use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: i128 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let ranked: Vec<i128> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let _: i128 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let player: Vec<i128> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    for n in climbing(ranked, player) {
        println!("{}", n);
    }
}

fn climbing(ranked: Vec<i128>, player: Vec<i128>) -> Vec<usize> {
    let mut nranked = ranked.clone();
    nranked.dedup();

    let mut res = Vec::new();
    for &p in &player {
        let i = bi_search(p, 0, nranked.len() - 1, &nranked);
        let rank = if i == nranked.len() { nranked.len() + 1 } else { i + 1 };
        res.push(rank);
    }

    res
}

fn bi_search(target: i128, lwi: usize, upi: usize, arr: &Vec<i128>) -> usize {
    if lwi > upi {
        return lwi;
    }
    let mid = (lwi + upi) / 2;
    if arr[mid] == target {
        return mid;
    }
    if arr[mid] > target {
        return bi_search(target, mid + 1, upi, arr);
    } else {
        if mid > 0 {
            return bi_search(target, lwi, mid - 1, arr);
        } else {
            return lwi;
        }
    }
}
