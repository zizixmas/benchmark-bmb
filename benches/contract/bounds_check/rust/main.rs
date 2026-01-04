// Bounds Check benchmark
// Measures: Array access with bounds checking
// In Rust: Uses safe array access (with bounds checks)

fn safe_access(arr: &[i64], index: usize) -> i64 {
    arr[index]
}

fn sum_range(arr: &[i64], start: usize, end: usize) -> i64 {
    if start >= end {
        0
    } else {
        safe_access(arr, start) + sum_range(arr, start + 1, end)
    }
}

fn sum_array(arr: &[i64]) -> i64 {
    let mut sum = 0i64;
    for i in 0..arr.len() {
        sum += arr[i];
    }
    sum
}

fn main() {
    let mut arr = [0i64; 10000];
    for i in 0..10000 {
        arr[i] = (i % 100) as i64;
    }

    let mut total = 0i64;
    for _ in 0..100 {
        total += sum_array(&arr);
    }

    println!("{}", total);
}
