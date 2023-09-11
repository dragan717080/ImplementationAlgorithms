use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, k, q] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let queries: Vec<i32> = (0..q).map(|_| {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    }).collect();

    let result = rotate_arr(numbers, k);
    for pos in queries {
        println!("{}", result[pos as usize]);
    }
}

fn rotate_arr<T: Clone>(arr: Vec<T>, places: i32) -> Vec<T> {
    let mut res = arr.clone();

    for (i, v) in arr.clone().iter().enumerate() {
        let new_index = (i + (places as usize)) % arr.len();
        res[new_index] = v.clone();
    }

    res
}
