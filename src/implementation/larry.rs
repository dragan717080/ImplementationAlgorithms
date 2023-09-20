use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let _: usize = line.trim().parse().unwrap();
        
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let arr: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
        println!("{}", sortable(arr));
    }
}

fn sortable(arr: Vec<i32>) -> &'static str {
    let mut inversions = 0;
    for i in 0..(arr.len() - 1) {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                inversions += 1;
            }
        }
    }
    if inversions % 2 == 0 { "YES" } else { "NO" }
}
