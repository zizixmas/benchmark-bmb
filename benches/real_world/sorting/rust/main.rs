// Sorting benchmark
// Measures: Comparison-based sorting, data movement

fn quicksort(arr: &mut [i64], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            quicksort(arr, low, pivot - 1);
        }
        quicksort(arr, pivot + 1, high);
    }
}

fn partition(arr: &mut [i64], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn insertion_sort(arr: &mut [i64]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn merge_sort(arr: &mut [i64]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp);
    arr.copy_from_slice(&temp);
}

fn merge(left: &[i64], right: &[i64], result: &mut [i64]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn is_sorted(arr: &[i64]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

fn generate_random(seed: u64, index: usize) -> i64 {
    let a = 1103515245u64;
    let c = 12345u64;
    let m = 1u64 << 31;
    (((seed.wrapping_mul(a).wrapping_add(c).wrapping_add(index as u64)) % m) % 10000) as i64
}

fn main() {
    let size = 5000;
    let seed = 12345u64;
    let mut checksum = 0i64;

    // Test quicksort
    for iter in 0..10 {
        let mut arr: Vec<i64> = (0..size).map(|i| generate_random(seed + iter as u64, i)).collect();
        let high = arr.len() - 1;
        quicksort(&mut arr, 0, high);
        if is_sorted(&arr) {
            checksum += 1;
        }
        checksum += arr[0] + arr[size - 1];
    }

    // Test merge_sort
    for iter in 0..10 {
        let mut arr: Vec<i64> = (0..size).map(|i| generate_random(seed + iter as u64 + 100, i)).collect();
        merge_sort(&mut arr);
        if is_sorted(&arr) {
            checksum += 1;
        }
        checksum += arr[0] + arr[size - 1];
    }

    println!("{}", checksum);
}
