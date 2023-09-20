use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<usize>().unwrap());
        let [n, k] = [numbers.next().unwrap(), numbers.next().unwrap()];
        
        let res = absolute_permutation(n, k);

        println!("{}", res.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn absolute_permutation(n: usize, k: usize) -> Vec<isize> {
    if k == 0 {
        // When k is 0, return a range from 1 to n
        return (1..=n as isize).collect();
    }
    if n % (2 * k) != 0 {
        // If n is not divisible by 2*k, return [-1]
        return vec![-1];
    }
    
    let mut ans = vec![-1; n];
    let mut si = k;
    let mut sj = 0;
    let mut v = 1;
    
    for _ in 0..(n / (2 * k)) {
        for _ in 0..k {
            ans[si] = v;
            v += 1;
            si += 1;
        }
        for _ in 0..k {
            ans[sj] = v;
            v += 1;
            sj += 1;
        }
        
        si += k;
        sj += k;
    }
    
    ans
}
