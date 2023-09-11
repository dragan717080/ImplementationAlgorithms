use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: u32 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let p: String = line.trim().to_owned();
    let mut lvl = 0;
    let mut res = 0;

    for c in p.chars() {
        if lvl == -1 && c == 'U' {
            res += 1;
        }

        lvl += if c == 'U' { 1 } else { -1 };
    }

    println!("{}", res);
}
