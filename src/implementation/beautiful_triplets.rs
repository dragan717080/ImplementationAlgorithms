use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, d] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let arr: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    println!("{}", find_triplets(d, &arr));
}

fn find_triplets(d: i32, arr: &[i32]) -> i32 {
    let mut res = 0;

    if arr.len() < 3 {
        return 0;
    }
    
    for i in 0..(arr.len() - 2) {
        for j in (i + 1)..(arr.len() - 1) {
            for k in (i + 2)..(arr.len()) {
                if arr[j] - arr[i] == d && arr[k] - arr[j] == d {
                    res += 1;
                }
            }
        }
    }

    res
}
