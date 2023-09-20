use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<usize>().unwrap());
    let [n, k] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let c: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    println!("{}", find_clouds(n, k, c));
}

fn find_clouds(n: usize, k: usize, mut c: Vec<i32>) -> i32 {
    if n % k != 0 {
        let mut mod_inverse = n + 1;

        while mod_inverse % k != 1 {
            mod_inverse += n;
        }

        c = c.iter().cloned().cycle().take(mod_inverse).collect();
    }
    else {
        c.push(c[0]);
    }

    let mut e = 100;

    for i in ((k as usize)..c.len()).step_by(k as usize) {
        e -= 1 + c[i] * 2;
    }

    e
}
