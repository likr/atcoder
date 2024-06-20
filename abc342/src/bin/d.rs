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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = HashMap::new();
    let mut zero_count = 0;
    for i in 0..n {
        if a[i] == 0 {
            zero_count += 1;
            continue;
        }
        let mut ai = a[i];
        let mut p_count = HashMap::new();
        for p in 2.. {
            if p * p > a[i] {
                break;
            }
            while ai % p == 0 {
                *p_count.entry(p).or_insert(0) += 1;
                ai /= p;
            }
        }
        if ai > 1 {
            *p_count.entry(ai).or_insert(0) += 1;
        }
        let mut x = 1;
        for (p, &c) in p_count.iter() {
            if c % 2 == 1 {
                x *= p
            }
        }
        *count.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0usize;
    if zero_count > 0 {
        ans += zero_count * (n - zero_count);
        ans += zero_count * (zero_count - 1) / 2;
    }
    for &c in count.values() {
        ans += c * (c - 1) / 2;
    }
    println!("{}", ans);
}
