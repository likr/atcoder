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
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        p: usize,
    }
    let mut q = p;
    let mut prime_factors = HashMap::new();
    for d in 2.. {
        if d * d > p {
            break;
        }
        while q % d == 0 {
            *prime_factors.entry(d).or_insert(0) += 1;
            q /= d;
        }
    }
    if q > 1 {
        *prime_factors.entry(q).or_insert(0) += 1;
    }
    let mut result = 1usize;
    for (&f, &c) in &prime_factors {
        if c >= n {
            result *= f.pow((c / n) as u32);
        }
    }
    println!("{}", result);
}
