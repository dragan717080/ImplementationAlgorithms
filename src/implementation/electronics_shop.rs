use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [b, _, _] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let keyboards: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let drives: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let permutations = find_permutations_of_lists(keyboards, drives);
    println!("{}", get_money_spent(permutations, b));
}

fn find_permutations_of_lists(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<[i32; 2]> {
    let mut res: Vec<[i32; 2]> = vec![];

    for number1 in arr1 {
        for number2 in &arr2 {
            res.push([number1, *number2]);
        }
    }

    res
}

fn get_money_spent(prices: Vec<[i32; 2]>, b: i32) -> i32 {
    let mut total_prices: Vec<i32> = prices.iter().map(|prices| prices.iter().sum()).collect();
    total_prices.retain(|&price| price <= b);

    return match total_prices.iter().max() {
        Some(n) => *n,
        None => -1,
    };
}
