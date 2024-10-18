// insertion sort function
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

// inertion sort function using recursion (Divide and Conquer)
fn insertion_sort_recursive(array: &mut [i32], l: usize, h: usize) {
    if l >= h {
        return;
    }
    insertion_sort_recursive(array, l, h - 1);
    let key = array[h];
    let mut j = h;
    while j > l && array[j - 1] > key {
        array[j] = array[j - 1];
        j -= 1;
    }
    array[j] = key;
}

// merge sort function using recursion (Divide and Conquer)
fn merge_sort(array: &mut [i32], l: usize, h: usize) {
    if l >= h {
        return;
    }
    let m = l + (h - l) / 2;
    merge_sort(array, l, m);
    merge_sort(array, m + 1, h);
    merge(array, l, m, h);
}

// merge function for merge sort
fn merge(array: &mut [i32], l: usize, m: usize, h: usize) {
    let n1 = m - l + 1;
    let n2 = h - m;
    let mut left = vec![0; n1];
    let mut right = vec![0; n2];
    for i in 0..n1 {
        left[i] = array[l + i];
    }
    for i in 0..n2 {
        right[i] = array[m + 1 + i];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = l;
    while i < n1 && j < n2 {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < n1 {
        array[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < n2 {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
}
fn main() {
    // Code for insertion sort algorithm usinng hoare partitioning and lazy pivot selection at array[l]
    let mut array = [12, 11, 13, 5, 6, 7];
    let n = array.len();
    insertion_sort(&mut array, 0, n - 1);
    println!("Sorted array: {:?}", array);

    // Code for insertion sort algorithm using insertion sort (iterative) at array[l]
    let mut array = [12, 11, 13, 5, 6, 7];
    let n = array.len();
    insertion_sort_recursive(&mut array, 0, n - 1);
    println!("Sorted array: {:?}", array);

    // Code for merge sort algorithm using divide and conquer
    let mut array = [12, 11, 13, 5, 6, 7];
    let n = array.len();
    merge_sort(&mut array, 0, n - 1);
    println!("Sorted array: {:?}", array);
}
