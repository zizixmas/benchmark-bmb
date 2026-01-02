#include <stdio.h>

// Bounds Check - contract optimization benchmark
// In C: runtime bounds checking required
// Measures: Array access overhead with bounds checking

#define SIZE 10000
#define ITERATIONS 1000

int array[SIZE];

int safe_access(int* arr, int size, int index) {
    if (index < 0 || index >= size) {
        return -1; // Bounds check required at runtime
    }
    return arr[index];
}

long long sum_array(int* arr, int size) {
    long long sum = 0;
    for (int i = 0; i < size; i++) {
        sum += safe_access(arr, size, i);
    }
    return sum;
}

int main() {
    // Initialize array
    for (int i = 0; i < SIZE; i++) {
        array[i] = i + 1;
    }

    long long total = 0;
    for (int iter = 0; iter < ITERATIONS; iter++) {
        total += sum_array(array, SIZE);
    }

    printf("Sum: %lld\n", total);
    return 0;
}
