use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t: u64 = line.trim().parse().unwrap();
    
    for _ in 0..t {

        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<u64>().unwrap());
        let [b, w] = [numbers.next().unwrap(), numbers.next().unwrap()];
        
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        let mut numbers = line.split_whitespace().map(|c| c.parse::<u64>().unwrap());
        let [bc, wc, z] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];

        println!("{}", taum(b, w, bc, wc, z));
    }
}

fn taum(b: u64, w: u64, bc: u64, wc: u64, z: u64) -> u64 {
    if bc > wc + z {
        return b * (wc + z) + w * wc;
    }
    else if wc > bc + z {
        return w * (bc + z) + b * bc;
    }
    else {
        return b * bc + w * wc;
    }
}
