fn main() {
    let mut arr = vec![1, 2, 3, 4];
    let permutations = permutations(&mut arr);

    for perm in permutations {
        println!("{:?}", perm);
    }
}

fn permutations(arr: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current_permutation = arr.clone();

    result.push(current_permutation.clone());

    while next_permutation(&mut current_permutation) {
        result.push(current_permutation.clone());
    }

    result
}

fn next_permutation(arr: &mut Vec<i32>) -> bool {
    let len = arr.len();
    let mut i = len as i32 - 2;

    while i >= 0 && arr[i as usize] >= arr[(i + 1) as usize] {
        i -= 1;
    }

    if i < 0 {
        // No next permutation; it's the last one.
        return false;
    }

    let mut j = len - 1;

    while arr[j] <= arr[i as usize] {
        j -= 1;
    }

    arr.swap(i as usize, j);
    arr[(i + 1) as usize..].reverse();

    true
}
