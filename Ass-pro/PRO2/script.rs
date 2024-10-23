use rand::Rng;
use std::time::Instant;

/// Insertion sort algorithm
fn insertion_sort(array: &mut [i32], l: usize, h: usize) {
    for i in l + 1..=h {
        let key = array[i];
        let mut j = i;
        while j > l && array[j - 1] > key {
            array[j] = array[j - 1];
            j -= 1;
        }
        array[j] = key;
    }
}

/// Merge two sorted subarrays into one sorted array
fn merge(left: &[i32], right: &[i32], array: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        array[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
}

/// Hybrid sort algorithm
fn hybrid_sort(array: &mut [i32], l: usize, h: usize, k: usize) {
    if h - l + 1 <= k {
        insertion_sort(array, l, h);
    } else {
        let mid = l + (h - l) / 2;
        hybrid_sort(array, l, mid, k);
        hybrid_sort(array, mid + 1, h, k);
        
        let left = array[l..=mid].to_vec();
        let right = array[mid + 1..=h].to_vec();
        merge(&left, &right, &mut array[l..=h]);
    }
}

fn main() {
    let k_values = [5, 10, 20, 50, 100];
    let n_values = [100, 1000, 5000, 10000, 20000];

    for &k in &k_values {
        for &n in &n_values {
            let mut array: Vec<i32> = (0..n).map(|_| rand::thread_rng().gen_range(0..10000)).collect();
            let mut array_copy = array.clone();

            let start = Instant::now();
            hybrid_sort(&mut array, 0, n - 1, k);
            let duration = start.elapsed();
            println!("HybridSort with K = {} and n = {} took: {:?}", k, n, duration);

            array_copy.sort();
            assert_eq!(array, array_copy, "The array is not sorted correctly!");
        }
    }
}