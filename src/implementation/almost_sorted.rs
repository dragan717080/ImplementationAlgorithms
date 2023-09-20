use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let _: usize = line.trim().parse().unwrap();
    
    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    can_sort(&mut arr);
}

fn can_sort(arr: &mut Vec<i32>) {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();

    let differences: Vec<(usize, (i32, i32))> = arr
        .iter()
        .zip(sorted_arr.iter())
        .enumerate()
        .filter(|&(_, (a, b))| a != b)
        .map(|(i, (a, b))| (i, (*a, *b)))
        .collect();

    if differences.len() == 2
        && arr[differences[0].0] == sorted_arr[differences[1].0]
        && arr[differences[1].0] == sorted_arr[differences[0].0]
    {
        println!("yes");
        println!("swap {} {}", differences[0].0 + 1, differences[1].0 + 1);
    } else {
        let start = differences[0].0;
        let end = differences[differences.len() - 1].0;

        let mut sublist_to_reverse = arr[start..=end].to_vec();
        sublist_to_reverse.reverse();

        if sublist_to_reverse == sorted_arr[start..=end].to_vec() {
            println!("yes");
            println!("reverse {} {}", start + 1, end + 1);
        } else {
            println!("no");
        }
    }
}