use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, _] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<i32> = line
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let b: Vec<i32> = line
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let factors_b: Vec<i32> = find_common_factors(&b);

    let mut res = 0;

    for factor_b in factors_b {
        let all_a_are_factors = a.iter().filter(|n| factor_b >= **n && factor_b % **n == 0).count()== a.len();
        if all_a_are_factors {
            res += 1;
        }
    }

    println!("{}", res);
}

fn find_common_factors(numbers: &Vec<i32>) -> Vec<i32> {
    if numbers.is_empty() {
        return Vec::new();
    }

    // Find the factors of the first number
    let mut factors = find_factors(numbers[0]);

    // Iterate through the rest of the numbers
    for &num in numbers.iter().skip(1) {
        // Find the factors of the current number
        let num_factors = find_factors(num);

        // Keep only the factors that are common with the previously found factors
        factors.retain(|&factor| num_factors.contains(&factor));
    }

    factors
}

fn find_factors(num: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    for i in 1..=num {
        if num % i == 0 {
            factors.push(i);
        }
    }
    factors
}
