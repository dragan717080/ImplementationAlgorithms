use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let _: i32 = line.trim().parse().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let [mut min_times, mut max_times] = [0, 0];
    let [mut min, mut max] = [numbers[0], numbers[0]];

    for number in numbers {
        if number < min {
            min = number;
            min_times += 1;
        } else if number > max {
            max = number;
            max_times += 1;
        }
    }

    println!("{} {}", max_times, min_times);
}
