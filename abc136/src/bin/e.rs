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

pub fn factors(n: usize) -> Vec<usize> {
    let mut result = vec![];
    for d in 1.. {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            result.push(d);
            result.push(n / d);
        }
    }
    result.sort();
    result.dedup();
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    let sum_a = a.iter().sum::<usize>();
    let factors = factors(sum_a);
    debug!(factors);

    let mut result = 0;
    for &x in &factors {
        let mut r = a.iter().map(|&ai| ai % x).collect::<Vec<_>>();
        r.sort();
        let mut minus = vec![0; n + 1];
        let mut plus = vec![0; n + 1];
        for i in 0..n {
            minus[i + 1] = minus[i] + r[i];
        }
        for i in (0..n).rev() {
            plus[i] = plus[i + 1] + x - r[i];
        }
        debug!(x);
        debug!(minus);
        debug!(plus);
        for i in 0..=n {
            if minus[i] == plus[i] && minus[i] <= k {
                result = x;
                break;
            }
        }
    }
    println!("{}", result);
}
