use num_bigint::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn solve(
    n: isize,
    a: &BigInt,
    b: &BigInt,
    c: &BigInt,
    d: &BigInt,
    cache: &mut HashMap<isize, BigInt>,
) -> BigInt {
    if let Some(x) = cache.get(&n) {
        x.clone()
    } else {
        // eprintln!("{}", n);
        let k = 4;
        let m0 = max(0, n - k);
        let m1 = n + k;
        let mut x = n * d;
        for m in m0..=m1 {
            if m % 2 == 0 && m / 2 < n {
                x = min(x, (n - m).abs() * d + a + solve(m / 2, a, b, c, d, cache));
            }
            if m % 3 == 0 && m / 3 < n {
                x = min(x, (n - m).abs() * d + b + solve(m / 3, a, b, c, d, cache));
            }
            if m % 5 == 0 && m / 5 < n {
                x = min(x, (n - m).abs() * d + c + solve(m / 5, a, b, c, d, cache));
            }
        }
        cache.insert(n, x);
        cache[&n].clone()
    }
}

fn main() {
    input! {
        t: usize,
        nabcd: [(isize, isize, isize, isize, isize); t],
    }
    for &(n, a, b, c, d) in &nabcd {
        let mut cache = HashMap::new();
        cache.insert(0, 0.to_bigint().unwrap());
        let result = solve(
            n,
            &a.to_bigint().unwrap(),
            &b.to_bigint().unwrap(),
            &c.to_bigint().unwrap(),
            &d.to_bigint().unwrap(),
            &mut cache,
        );
        // eprintln!("{:?}", cache);
        println!("{}", result);
    }
}
