const LEFT_TO_RIGHT: bool = true;
const RIGHT_TO_LEFT: bool = false;

fn search_arr(a: &Vec<usize>, n: usize, mobile: usize) -> usize {
    for i in 0..n {
        if a[i] == mobile {
            return i + 1;
        }
    }
    0
}

fn get_mobile(a: &Vec<usize>, dir: &Vec<bool>, n: usize) -> usize {
    let mut mobile_prev = 0;
    let mut mobile = 0;
    
    for i in 0..n {
        if dir[a[i] - 1] == RIGHT_TO_LEFT && i != 0 {
            if a[i] > a[i - 1] && a[i] > mobile_prev {
                mobile = a[i];
                mobile_prev = mobile;
            }
        }
        
        if dir[a[i] - 1] == LEFT_TO_RIGHT && i != n - 1 {
            if a[i] > a[i + 1] && a[i] > mobile_prev {
                mobile = a[i];
                mobile_prev = mobile;
            }
        }
    }
    
    if mobile == 0 && mobile_prev == 0 {
        0
    } else {
        mobile
    }
}

fn print_one_perm(a: &mut Vec<usize>, dir: &mut Vec<bool>, n: usize) {
    let mobile = get_mobile(a, dir, n);
    let pos = search_arr(&a, n, mobile);

    if dir[a[pos - 1] - 1] == RIGHT_TO_LEFT {
        a.swap(pos - 1, pos - 2);
    } else if dir[a[pos - 1] - 1] == LEFT_TO_RIGHT {
        a.swap(pos, pos - 1);
    }

    for i in 0..n {
        if a[i] > mobile {
            if dir[a[i] - 1] == LEFT_TO_RIGHT {
                dir[a[i] - 1] = RIGHT_TO_LEFT;
            } else if dir[a[i] - 1] == RIGHT_TO_LEFT {
                dir[a[i] - 1] = LEFT_TO_RIGHT;
            }
        }
    }
}

fn fact(n: usize) -> usize {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }
    res
}

fn generate_permutations(n: usize) -> Vec<Vec<usize>> {
    let mut permutations = Vec::new();
    let mut a: Vec<usize> = (1..=n).collect();
    let mut dir: Vec<bool> = vec![RIGHT_TO_LEFT; n];

    permutations.push(a.clone());

    for _ in 1..fact(n) {
        print_one_perm(&mut a, &mut dir, n);
        permutations.push(a.clone());
    }

    permutations
}

fn main() {
    let n = 4;
    let permutations = generate_permutations(n);

    for perm in permutations {
        println!("{:?}", perm);
    }
}
