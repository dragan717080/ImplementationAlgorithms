use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap_or_else(|_| panic!("Failed to parse integer"));

    let mut numbers: Vec<i32> = (0..n).map(|_| {
        line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<i32>().unwrap()
    }).collect();
    
    for number in &mut numbers {
        if *number < 38 {
            continue;
        }

        if *number % 5 == 3 || *number % 5 == 4 {
            *number += 5 - *number % 5;
        }
    };

    for n in numbers {
        println!("{}", n);
    }
}
