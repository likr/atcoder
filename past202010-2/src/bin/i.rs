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
const INF: i64 = std::i64::MAX / 4;
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
        a: [i64; n],
    }
    let s = a.iter().sum::<i64>();

    let mut b = a.clone();
    for &ai in &a {
        b.push(ai);
    }

    let mut acc_b = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        acc_b[i + 1] = acc_b[i] + b[i];
    }
    debug!(acc_b);

    let mut result = INF;
    let mut j = 1;
    for i in 0..n {
        while j + 1 < i + n
            && ((s - 2 * (acc_b[j + 1] - acc_b[i])).abs() < (s - 2 * (acc_b[j] - acc_b[i])).abs())
        {
            j += 1;
        }
        debug!(i, j);
        result = min(result, (s - 2 * (acc_b[j] - acc_b[i])).abs());
    }
    println!("{}", result);
}
