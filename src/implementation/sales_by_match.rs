use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: u32 = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let arr: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    let mut unmatched_elements: Vec<u32> = Vec::new();
    let mut matches: u32 = 0;

    for item in &arr {
        if !(unmatched_elements.contains(&item)) {
            unmatched_elements.push(*item);
        }
        else {
            let i = unmatched_elements.iter().position(|x| x == item).unwrap();
            unmatched_elements.remove(i);
            matches += 1;
        }
    }

    println!("{}", matches);
}
