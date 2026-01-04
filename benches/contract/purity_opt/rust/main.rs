// Purity optimization benchmark
// Measures: Common subexpression elimination, memoization potential
// In Rust: Compiler can optimize pure functions

#[inline(never)]
fn pure_compute(x: i64, y: i64) -> i64 {
    let a = x * x + y * y;
    let b = x * y * 2;
    a + b + (x + y) * (x - y)
}

fn redundant_calls(n: i64) -> i64 {
    let mut sum = 0i64;

    for i in 0..n {
        // Intentionally redundant: same computation multiple times
        let a = pure_compute(i, i + 1);
        let b = pure_compute(i, i + 1);  // Same as 'a'
        let c = pure_compute(i, i + 1);  // Same as 'a'
        sum += a + b + c;
    }

    sum
}

fn expensive_invariant(n: i64, base: i64) -> i64 {
    let mut sum = 0i64;

    for i in 0..n {
        // Loop-invariant computation
        let invariant = pure_compute(base, base + 1);
        sum += invariant + i;
    }

    sum
}

fn main() {
    let result1 = redundant_calls(10000);
    let result2 = expensive_invariant(10000, 42);

    println!("{}", result1 + result2);
}
