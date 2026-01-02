# benchmark-bmb

> Standard Benchmarking Suite for BMB Language

BMB 언어의 표준 벤치마크 스위트. C, Rust, BMB 간 성능 비교를 제공합니다.

## Goal

**BMB >= C -O3** (모든 케이스)
**BMB > C -O3** (계약 활용 케이스)

## Current Status: v0.1

### Implemented Benchmarks

| Category | Benchmark | C | BMB | Description |
|----------|-----------|---|-----|-------------|
| compute | fibonacci | ✅ | ✅ | Recursive function calls |
| compute | n_body | ✅ | 🔧 | N-body simulation (pending f64) |
| contract | bounds_check | ✅ | ✅ | Bounds check elimination |

## Benchmark Categories

### Compute-Intensive

| Benchmark | Description | Measures |
|-----------|-------------|----------|
| `fibonacci` | Recursive Fibonacci | Integer ops, function calls |
| `n-body` | N-body simulation | FP arithmetic, SIMD |
| `mandelbrot` | Fractal generation | SIMD, parallelism |
| `spectral-norm` | Matrix operations | Linear algebra |

### Contract-Optimized (BMB-specific)

| Benchmark | Description | Contract Benefit |
|-----------|-------------|------------------|
| `bounds-check` | Array operations | pre로 경계검사 제거 |
| `null-check` | Optional handling | NonNull 타입으로 제거 |
| `purity-opt` | Pure functions | 순수성 기반 최적화 |

## Directory Structure

```
benchmark-bmb/
├── README.md
├── benches/
│   ├── compute/
│   │   ├── fibonacci/
│   │   │   ├── c/main.c
│   │   │   └── bmb/main.bmb
│   │   └── n_body/
│   │       ├── c/main.c
│   │       └── bmb/main.bmb
│   └── contract/
│       └── bounds_check/
│           ├── c/main.c
│           └── bmb/main.bmb
├── runner/
│   ├── Cargo.toml
│   └── src/main.rs
└── results/
```

## Running Benchmarks

```bash
# Build runner
cd runner
cargo build --release

# Run all benchmarks
./target/release/benchmark-bmb run --all

# Run specific category
./target/release/benchmark-bmb run --category compute

# Run single benchmark
./target/release/benchmark-bmb run fibonacci

# List available benchmarks
./target/release/benchmark-bmb list

# Create new benchmark
./target/release/benchmark-bmb new my_benchmark --category compute
```

## Output Format

```
=== BMB Benchmark Suite ===

Running: fibonacci

  Language     Median (ms)     Min (ms)     Max (ms)   Relative
  ------------------------------------------------------------
  C                 850.23       845.12       860.45      1.00x
  BMB               855.67       850.01       865.23         ✓

BMB is within 1% of C -O3
```

## Benchmark Requirements

### Implementation Guidelines

1. **Identical Algorithm**: Same algorithm across all languages
2. **No External Libraries**: Standard library only
3. **Fair Optimization**: Language-appropriate optimizations allowed
4. **Contracts in BMB**: Use pre/post where applicable

### Measurement

- **Warm-up**: 2 iterations before measurement
- **Iterations**: 5 measurements, median reported
- **Metrics**: Wall time, relative performance

## Runner CLI Commands

| Command | Description |
|---------|-------------|
| `run` | Run benchmarks |
| `list` | List available benchmarks |
| `new` | Create new benchmark scaffold |
| `compare` | Compare languages for a benchmark |
| `validate` | Validate implementations produce same output |
| `report` | Generate benchmark report |

## Roadmap

| Version | Features | Status |
|---------|----------|--------|
| v0.1 | Basic runner, compute benchmarks | ✅ |
| v0.2 | Memory benchmarks, comparison reports | 계획 |
| v0.3 | Real-world benchmarks, CI integration | 계획 |
| v0.4 | Contract-optimized benchmarks (full) | 계획 |
| v0.5 | Dashboard, regression detection | 계획 |

## Contributing

1. Fork the repository
2. Add benchmark implementation
3. Validate correctness
4. Submit PR with benchmark results

## License

MIT License
