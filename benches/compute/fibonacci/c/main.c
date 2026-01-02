#include <stdio.h>

// Fibonacci - compute intensive benchmark
// Measures: recursive function calls, integer arithmetic

long long fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
    int n = 35;
    long long result = fibonacci(n);
    printf("fibonacci(%d) = %lld\n", n, result);
    return 0;
}
