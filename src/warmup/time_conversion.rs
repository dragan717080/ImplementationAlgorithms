use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    
    let parts: Vec<&str> = line.trim().split(":").collect();
    
    assert_eq!(parts.len(), 3);

    let mut h = parts[0].parse::<i32>().unwrap();
    let m = parts[1];

    let mut s = parts[2].clone().to_string();
    let (s, time_format) = s.split_at_mut(parts[2].len() / 2);

    if time_format.chars().nth(0).unwrap() == 'P' && h != 12 {
        h += 12;
    };

    if h == 12 {
        h = if time_format.chars().nth(0).unwrap() == 'P' { 12 } else { 0 };
    }

    let h_str: String = if h < 10 { format!("0{}", h.to_string()) } else { h.to_string() };

    println!("{}:{}:{}", h_str, m, s);
}
