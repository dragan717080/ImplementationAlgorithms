use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [n, _] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut stations: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
    stations.sort();

    let result = flatland_stations(n, stations);
    println!("{}", result);
}

fn flatland_stations(n: i32, stations: Vec<i32>) -> i32 {
    let mut result = stations[0];
    for i in 0..stations.len() - 1 {
        result = result.max((stations[i + 1] - stations[i]) / 2);
    }
    
    result.max(n - 1 - stations[stations.len() - 1])
}
