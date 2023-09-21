use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let s: String = line.trim().to_owned();

        let result = solution(s);
        println!("{}", result.unwrap_or_else(|| "no answer".to_owned()));
    }
}


fn solution(s: String) -> Option<String> {
    let mut result: Vec<_> = s.chars().collect();

    let (idx_to_exchange, ch_to_exchange) = result.windows(2).enumerate().map(|(window_idx, window)| ((window_idx, window[0]), (window_idx+1, window[1]))).rev().find(|((_, left_ch), (_, right_ch))| left_ch < right_ch)?.0;

    let right_idx_to_exchange = idx_to_exchange + 1 + result[idx_to_exchange+1..].iter().enumerate().filter(|(_, &ch)| ch > ch_to_exchange).min_by(|(_, left_ch), (_, right_ch)| left_ch.cmp(right_ch))?.0;

    result.swap(idx_to_exchange, right_idx_to_exchange);
    result[idx_to_exchange+1..].sort();

    Some(result.into_iter().collect())
}
