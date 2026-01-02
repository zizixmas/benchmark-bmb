# benchmark-bmb

> Standard Benchmarking Suite for BMB Language

BMB 언어의 표준 벤치마크 스위트. C, Rust, BMB 간 성능 비교를 제공합니다.

## Goal

**BMB >= C -O3** (모든 케이스)
**BMB > C -O3** (계약 활용 케이스)

## Benchmark Categories

### Compute-Intensive

| Benchmark | Description | Measures |
|-----------|-------------|----------|
| `n-body` | N-body simulation | FP arithmetic, SIMD |
| `mandelbrot` | Fractal generation | SIMD, parallelism |
| `spectral-norm` | Matrix operations | Linear algebra |
| `fannkuch` | Permutation | Integer ops, branching |
| `fasta` | Sequence generation | String ops |

### Memory-Intensive

| Benchmark | Description | Measures |
|-----------|-------------|----------|
| `binary-trees` | Tree alloc/dealloc | GC/allocator |
| `reverse-complement` | In-place reversal | Memory bandwidth |
| `k-nucleotide` | Hashtable | Collection performance |

### Real-World

| Benchmark | Description | Measures |
|-----------|-------------|----------|
| `json-parse` | JSON parsing | Parser performance |
| `regex-redux` | Regex matching | Regex engine |
| `http-throughput` | HTTP handling | I/O, concurrency |

### Contract-Optimized (BMB-specific)

| Benchmark | Description | Contract Benefit |
|-----------|-------------|------------------|
| `bounds-check-elim` | Array operations | pre로 경계검사 제거 |
| `null-check-elim` | Optional handling | NonNull 타입으로 제거 |
| `aliasing-proof` | Pointer operations | &mut 배타성 증명 |
| `purity-opt` | Pure functions | 순수성 기반 메모이제이션 |

## Directory Structure

```
benchmark-bmb/
├── README.md
├── benches/
│   ├── compute/
│   │   ├── n_body/
│   │   │   ├── c/          # C implementation
│   │   │   ├── rust/       # Rust implementation
│   │   │   └── bmb/        # BMB implementation
│   │   ├── mandelbrot/
│   │   └── ...
│   ├── memory/
│   │   ├── binary_trees/
│   │   └── ...
│   ├── realworld/
│   │   ├── json_parse/
│   │   └── ...
│   └── contract/           # BMB-specific optimizations
│       ├── bounds_check/
│       └── ...
├── runner/                 # Benchmark runner (Rust)
│   ├── Cargo.toml
│   └── src/
├── results/                # Historical results
│   └── YYYY-MM-DD/
└── dashboard/              # Web dashboard (planned)
```

## Running Benchmarks

```bash
# Install runner
cargo install --path runner

# Run all benchmarks
benchmark-bmb run --all

# Run specific category
benchmark-bmb run --category compute

# Run single benchmark
benchmark-bmb run n-body

# Compare languages
benchmark-bmb compare n-body --langs c,rust,bmb

# Generate report
benchmark-bmb report --format html --output results/
```

## Output Format

```
=== n-body (1,000,000 iterations) ===

| Language | Time (ms) | Memory (KB) | Relative |
|----------|-----------|-------------|----------|
| C -O3    |    142.3  |       1,024 |   1.00x  |
| Rust     |    145.7  |       1,048 |   1.02x  |
| BMB      |    139.8  |       1,016 |   0.98x  |  ✓

BMB is 1.8% faster than C -O3
```

## Benchmark Requirements

### Implementation Guidelines

1. **Identical Algorithm**: Same algorithm across all languages
2. **No External Libraries**: Standard library only
3. **Fair Optimization**: Language-appropriate optimizations allowed
4. **Contracts in BMB**: Use pre/post where applicable

### Measurement

- **Warm-up**: 3 iterations before measurement
- **Iterations**: 10 measurements, median reported
- **Environment**: Identical hardware, isolated execution
- **Metrics**: Wall time, peak memory, CPU cycles

## Adding New Benchmarks

```bash
# Create benchmark scaffold
benchmark-bmb new my_benchmark --category compute

# Implement in each language
# benches/compute/my_benchmark/{c,rust,bmb}/

# Validate implementations produce same output
benchmark-bmb validate my_benchmark

# Submit PR
```

## CI Integration

```yaml
# .github/workflows/benchmark.yml
name: Benchmark

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: lang-bmb/action-bmb@v1
      - run: benchmark-bmb run --all
      - run: benchmark-bmb report --publish
```

## Results Dashboard

Visit [bench.bmb-lang.org](https://bench.bmb-lang.org) for:
- Historical performance trends
- Cross-platform comparisons
- Regression detection alerts
- Detailed analysis reports

## Roadmap

| Version | Features |
|---------|----------|
| v0.1 | Basic runner, compute benchmarks |
| v0.2 | Memory benchmarks, comparison reports |
| v0.3 | Real-world benchmarks, CI integration |
| v0.4 | Contract-optimized benchmarks |
| v0.5 | Dashboard, regression detection |
| v1.0 | Full suite, automated publishing |

## Contributing

1. Fork the repository
2. Add benchmark implementation
3. Validate correctness
4. Submit PR with benchmark results

## License

MIT License
