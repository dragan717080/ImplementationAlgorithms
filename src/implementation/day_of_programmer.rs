use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let y: u32 = line.trim().parse().unwrap();

    if y == 1918 {
        println!("26.09.{}", y);
    } else {
        let d: u32 = if is_leap_year(y) { 2 } else { 3 };
        println!("1{}.09.{}", d, y);
    }
}

fn is_leap_year(y: u32) -> bool {
    let condition: bool = if y < 1919 { true } else { y % 100 != 0 };
    y % 4 == 0 && condition || y % 400 == 0
}
