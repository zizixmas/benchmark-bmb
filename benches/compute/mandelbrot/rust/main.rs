// Mandelbrot set benchmark
// Measures: complex arithmetic, iteration, bit manipulation

fn main() {
    let size = 200;
    let max_iter = 50;
    let mut sum = 0i64;

    for py in 0..size {
        for px in 0..size {
            let x0 = (px as f64) * 4.0 / (size as f64) - 2.0;
            let y0 = (py as f64) * 4.0 / (size as f64) - 2.0;

            let mut x = 0.0f64;
            let mut y = 0.0f64;
            let mut iteration = 0;

            while x * x + y * y <= 4.0 && iteration < max_iter {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }

            if iteration == max_iter {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
