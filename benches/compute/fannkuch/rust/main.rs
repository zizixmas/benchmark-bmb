// Fannkuch-redux benchmark
// Measures: permutation generation, array reversal

fn fannkuch(n: usize) -> i32 {
    let mut perm = vec![0i32; n];
    let mut perm1 = vec![0i32; n];
    let mut count = vec![0i32; n];

    for i in 0..n {
        perm1[i] = i as i32;
    }

    let mut max_flips = 0;
    let mut checksum = 0;
    let mut r = n;
    let mut perm_count = 0;

    loop {
        while r != 1 {
            count[r - 1] = r as i32;
            r -= 1;
        }

        for i in 0..n {
            perm[i] = perm1[i];
        }

        let mut flips = 0;
        loop {
            let k = perm[0] as usize;
            if k == 0 {
                break;
            }

            let k2 = (k + 1) / 2;
            for i in 0..k2 {
                perm.swap(i, k - i);
            }
            flips += 1;
        }

        max_flips = max_flips.max(flips);
        if perm_count % 2 == 0 {
            checksum += flips;
        } else {
            checksum -= flips;
        }

        loop {
            if r == n {
                println!("{}", checksum);
                return max_flips;
            }

            let perm0 = perm1[0];
            for i in 0..r {
                perm1[i] = perm1[i + 1];
            }
            perm1[r] = perm0;

            count[r] -= 1;
            if count[r] > 0 {
                break;
            }
            r += 1;
        }

        perm_count += 1;
    }
}

fn main() {
    let n = 10;
    let result = fannkuch(n);
    println!("Pfannkuchen({}) = {}", n, result);
}
