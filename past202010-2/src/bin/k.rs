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
const M: usize = 1000000000;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        k: usize,
        a: [[Usize1]; k],
        q: usize,
        b: [Usize1; q],
    }

    let mut inversion_number = vec![0; k];
    let mut count = vec![vec![0; 20]; k];
    for i in 0..k {
        for &aij in &a[i] {
            for y in aij + 1..20 {
                inversion_number[i] = (inversion_number[i] + count[i][y]) % M;
            }
            count[i][aij] += 1;
        }
    }
    debug!(inversion_number);
    debug!(count);

    let mut result = 0usize;
    let mut s = vec![0; 20];
    for &bi in &b {
        let c = &count[bi];
        result = (result + inversion_number[bi]) % M;
        for x in 0..20 {
            for y in x + 1..20 {
                result = (result + c[x] * s[y]) % M;
            }
        }
        for x in 0..20 {
            s[x] += c[x];
        }
    }
    println!("{}", result);
}
