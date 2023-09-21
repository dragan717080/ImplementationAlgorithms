use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let h: i32 = line.trim().parse().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();
    let m: i32 = line.trim().parse().unwrap();
    
    println!("{}", find_time(h, m));
}

fn find_time(h: i32, m: i32) -> String {
    let hours = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve"].to_vec();
    let mut minutes = hours.clone();
    minutes.extend(["thirteen", "fourteen", "fifteen","sixteen", "seventeen", "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three", "twenty four", "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine"].to_vec());

    let (mut hour, mut minute, mut hour_next, mut minute_next) = ("s".to_string(), "s".to_string(), "s".to_string(), "s".to_string());

    if m == 0 { 
        hour = hours[(h - 1) as usize].to_owned();
    }
    else if m <= 29 {
        hour = hours[(h - 1) as usize].to_owned();
        minute = minutes[(m - 1) as usize].to_owned();
    }
    else {
        if m == 30 {
            hour = hours[(h - 1) as usize].to_owned();
        }
        hour_next = if h != 12 { hours[h as usize].to_owned() } else { "one".to_owned() };
        minute_next = if m != 30 { minutes[(60 - m - 1) as usize].to_owned() } else { "s".to_owned() };
    }


    match m {
       0 => { return hour + " o' clock"; },
       1 => { return "one minute past ".to_string() + &hour },
       2..=14 | 16..=29 => { return minute + " minutes past " + &hour },
       15 => { return "quarter past ".to_string() + &hour },
       30 => { return "half past ".to_string() + &hour },
       3..=44 | 46..=58 => { return minute_next + " minutes to " + &hour_next },
       45 => { return "quarter to ".to_string() + &hour_next },
       59 => { return "one minute to ".to_string() + &hour_next }
       _ => { return "o'clock".to_string(); }
    }
}
