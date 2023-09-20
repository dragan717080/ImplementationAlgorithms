use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [_, k] = [numbers.next().unwrap(), numbers.next().unwrap()];

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let a: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

    println!("{}", workbook(k, &a));
}

fn workbook(k: i32, arr: &[i32]) -> i32 {
    let mut res = 0;
    let mut current_page = 0;

    for d in arr {
        let mut chapter_problems = 1;

        for _ in 0..(d / k) {
            current_page += 1;
            for chapter_problem in chapter_problems..(chapter_problems + k) {
                if current_page == chapter_problem {
                    res += 1;
                }
            }
            chapter_problems += k;
        }

        if d % k != 0 {
            current_page += 1;
            for chapter_problem in chapter_problems..(chapter_problems + d % k) {
                if current_page == chapter_problem {
                    res += 1;
                }
            }
        }
    }

    res
}
