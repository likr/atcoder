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

fn modpow(x: usize, y: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut count = vec![0; k + 1];
    for i in 1..=k {
        let c = k / i;
        count[i] = modpow(c, n);
    }
    // eprintln!("{:?}", count);

    let mut factors = HashSet::new();
    for i in (1..=k).rev() {
        factors.clear();
        let sqrt_i = (i as f64).sqrt() as usize;
        for d in 1..=sqrt_i {
            if i % d == 0 {
                factors.insert(d);
                factors.insert(i / d);
            }
        }
        factors.remove(&i);
        for &j in factors.iter() {
            count[j] = (count[j] + M - count[i]) % M;
        }
    }
    // eprintln!("{:?}", count);

    let mut s = 0usize;
    for i in 1..=k {
        s = (s + i * count[i]) % M;
    }

    println!("{}", s);
}
