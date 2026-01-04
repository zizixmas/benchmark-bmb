// Spectral norm benchmark
// Measures: matrix operations, eigenvalue approximation

fn a(i: usize, j: usize) -> f64 {
    1.0 / ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

fn multiply_av(n: usize, v: &[f64], av: &mut [f64]) {
    for i in 0..n {
        av[i] = 0.0;
        for j in 0..n {
            av[i] += a(i, j) * v[j];
        }
    }
}

fn multiply_atv(n: usize, v: &[f64], atv: &mut [f64]) {
    for i in 0..n {
        atv[i] = 0.0;
        for j in 0..n {
            atv[i] += a(j, i) * v[j];
        }
    }
}

fn multiply_atav(n: usize, v: &[f64], atav: &mut [f64], tmp: &mut [f64]) {
    multiply_av(n, v, tmp);
    multiply_atv(n, tmp, atav);
}

fn main() {
    let n = 100;
    let mut u = vec![1.0f64; n];
    let mut v = vec![0.0f64; n];
    let mut tmp = vec![0.0f64; n];

    for _ in 0..10 {
        multiply_atav(n, &u, &mut v, &mut tmp);
        multiply_atav(n, &v, &mut u, &mut tmp);
    }

    let mut vbv = 0.0f64;
    let mut vv = 0.0f64;

    for i in 0..n {
        vbv += u[i] * v[i];
        vv += v[i] * v[i];
    }

    let result = (vbv / vv).sqrt();
    println!("{:.9}", result);
}
