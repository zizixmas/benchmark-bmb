// Aliasing benchmark
// Measures: SIMD vectorization potential when aliasing is proven absent
// In Rust: Ownership system helps prove non-aliasing

fn add_arrays(a: &[i64], b: &[i64], result: &mut [i64]) {
    let n = a.len().min(b.len()).min(result.len());
    for i in 0..n {
        result[i] = a[i] + b[i];
    }
}

fn scale_array(arr: &mut [i64], factor: i64) {
    for i in 0..arr.len() {
        arr[i] *= factor;
    }
}

fn dot_product(a: &[i64], b: &[i64]) -> i64 {
    let mut sum = 0i64;
    let n = a.len().min(b.len());
    for i in 0..n {
        sum += a[i] * b[i];
    }
    sum
}

fn matrix_vector_multiply(matrix: &[i64], vector: &[i64], result: &mut [i64], rows: usize, cols: usize) {
    for i in 0..rows {
        result[i] = 0;
        for j in 0..cols {
            result[i] += matrix[i * cols + j] * vector[j];
        }
    }
}

fn main() {
    let size = 1000;
    let mut a = vec![0i64; size];
    let mut b = vec![0i64; size];
    let mut c = vec![0i64; size];

    for i in 0..size {
        a[i] = (i % 100) as i64;
        b[i] = ((i * 2) % 100) as i64;
    }

    let mut total = 0i64;

    for _ in 0..1000 {
        add_arrays(&a, &b, &mut c);
        scale_array(&mut c, 2);
        total += dot_product(&a, &c);
    }

    println!("{}", total);
}
