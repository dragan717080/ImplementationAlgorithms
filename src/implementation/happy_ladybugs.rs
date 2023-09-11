use std::{io, collections::HashMap};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let g: i32 = line.trim().parse().unwrap();
    
    for _ in 0..g {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let _: i32 = line.trim().parse().unwrap();
        
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let b = line.trim().to_string();

        let res = board(b);
        println!("{}", res);
    }
    
}

fn board(b: String) -> &'static str {
    let res: String = b.chars().into_iter().filter(|c| c.is_alphabetic()).collect();
    let empty_cells_exist = b.len() != res.len();
    
    let mut freqs: HashMap<char, i32> = HashMap::new();

    for c in res.chars() {
        let counter = freqs.entry(c).or_insert(0);
        *counter += 1;
    }

    if freqs.values().len() == 0 {
        return "YES";
    }

    let min = freqs.values().into_iter().min().unwrap().clone();
    if min == 0 {
        return "YES";
    }

    if !empty_cells_exist {
        return if is_entirely_repeated(&res) { "YES" } else { "NO" };
    }

    else {
        return if min != 1 { "YES" } else { "NO" };
    }
}

fn is_entirely_repeated(s: &str) -> bool {
    let mut prev_char = None;
    let mut group_len = 0;

    for current_char in s.chars() {
        if let Some(prev) = prev_char {
            if current_char == prev {
                group_len += 1;
            } else {
                if group_len < 2 {
                    return false;
                }
                group_len = 1;
            }
        } else {
            group_len = 1;
        }
        prev_char = Some(current_char);
    }

    group_len >= 2
}
