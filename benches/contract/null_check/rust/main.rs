// Null Check benchmark
// Measures: Optional value handling
// In Rust: Uses Option<T> with pattern matching

fn safe_divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn chain_divide(values: &[i64]) -> Option<i64> {
    if values.is_empty() {
        return None;
    }

    let mut result = values[0];
    for i in 1..values.len() {
        match safe_divide(result, values[i]) {
            Some(v) => result = v,
            None => return None,
        }
    }
    Some(result)
}

fn process_optional(opt: Option<i64>) -> i64 {
    match opt {
        Some(v) => v * 2,
        None => 0,
    }
}

fn main() {
    let mut sum = 0i64;

    for i in 0..100000 {
        let values = [1000000i64, (i % 10) + 1, (i % 5) + 1, (i % 3) + 1];
        let result = chain_divide(&values);
        sum += process_optional(result);
    }

    println!("{}", sum);
}
