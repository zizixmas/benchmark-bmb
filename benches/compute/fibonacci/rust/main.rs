// Fibonacci - compute intensive benchmark
// Measures: recursive function calls, integer arithmetic

fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let result = fibonacci(35);
    println!("{}", result);
}
