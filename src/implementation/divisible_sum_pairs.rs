use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();

    let mut result = 0;

    for i in 0..(numbers.len() - 1) {
        for j in (i + 1)..numbers.len() {
            if (numbers[i] + numbers[j]) % k == 0 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
