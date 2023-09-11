use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let p: u64 = line.trim().parse().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let q: u64 = line.trim().parse().unwrap();

    let res = find_kaprekars_in_range(p, q);
    let res_str: Vec<String> = res.iter().map(|&n| n.to_string()).collect();
    if res_str.len() == 0 {
        println!("INVALID RANGE");
    }
    else {
        println!("{}", res_str.join(" "));
    }
}

fn find_kaprekars_in_range(p: u64, q: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    for number in p..=q {
        let number_digits = number.to_string().len();
        let sq = number.pow(2);
        let sq_str = sq.to_string();

        // Right hand side is number_digits digits long
        let r = sq_str.chars().rev().take(number_digits).collect::<String>().chars().rev().collect::<String>();
        let l = if number > 3 { sq_str.chars().take(sq_str.len() - number_digits).collect::<String>() } else { 0.to_string() };
        if l.parse::<u64>().unwrap() + r.parse::<u64>().unwrap() == number {
            res.push(number);
        }
    }

    res
}
