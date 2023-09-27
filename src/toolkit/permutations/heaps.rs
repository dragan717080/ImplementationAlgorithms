use rand::Rng;

fn main() {
    let arr: Vec<i32> = (0..7).map(|_| rand::thread_rng().gen_range(1..=5000)).collect();
    let permuted_arr: Vec<Vec<i32>> = heaps_algorithm(arr.len(), &mut (0..arr.len()).collect(), &mut Vec::new()).iter().map(|indices| indices.iter().map(|i| arr[*i]).collect()).collect();
    println!("{:?}", permuted_arr);
}

fn heaps_algorithm<T: Clone>(n: usize, elements: &mut Vec<T>, res: &mut Vec<Vec<T>>) -> Vec<Vec<T>> {
    if n == 1 {
        res.push(elements.clone());
        return res.to_vec();
    }

    for i in 0..n - 1 {
        heaps_algorithm(n - 1, elements, res);
        if n % 2 == 0 {
            elements.swap(i, n - 1);
        }
        else {
            elements.swap(0, n - 1);
        };
    }

    heaps_algorithm(n - 1, elements, res)
}
