use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let arr: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
    println!("{}", find_subsets(arr, k));
}

fn find_subsets(arr: Vec<i32>, k: i32) -> i32 {
    let mut simple = vec![0; k as usize];
    let mut count = 0;

    for &item in arr.iter() {
        simple[(item % k) as usize] += 1;
    }

    if simple[0] > 0 {
        count += 1;
    }

    for i in 1..(k / 2 + 1) {
        let remainder = i;
        let opposite_remainder = k - i;

        if remainder == opposite_remainder {
            count += 1;
        } else {
            count += simple[remainder as usize].max(simple[opposite_remainder as usize]);
        }
    }

    count
}
