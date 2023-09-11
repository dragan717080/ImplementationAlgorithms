use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut s = line.trim().to_owned();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let t = line.trim().to_owned();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let k: usize = line.trim().parse().unwrap();

    let res = append_and_delete(&mut s, &t, k);
    println!("{}", res);
}

fn append_and_delete(s: &str, t: &str, k: usize) -> String {
    let mut i: u32 = 0;
    let mut s = s.to_string();

    while i < k as u32 {
        let condition = t.len() >= s.len();
        if condition && s == t[0..s.len()] && ((k - i as usize) == t.len() - s.len()) {
            return "Yes".to_string();
        }
        else {
            s.pop();
            i += 1;
        }
    }

    "No".to_string()
}
