use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    let start_people = 5;
    let res = advertise(n, start_people, None);
    println!("{}", res);
}

fn advertise(n: i32, start_people: i32, likes: Option<i32>) -> i32 {
    if n == 1 {
        return (start_people / 2) + likes.unwrap();
    }

    let likes_count = match likes {
        Some(n) => n + start_people / 2,
        None => start_people / 2
    };

    advertise(n - 1, (start_people / 2) * 3, Some(likes_count))
}
