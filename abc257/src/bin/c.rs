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
        s: Chars,
        w: [usize; n],
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| (w[i], Reverse(s[i])));
    let mut count0 = vec![0; n + 1];
    for i in 0..n {
        count0[i + 1] = count0[i];
        if s[indices[i]] == '0' {
            count0[i + 1] += 1;
        }
    }
    let mut result = n - count0[n];
    for i in 0..n {
        result = max(
            result,
            count0[i + 1] + (n - i - 1 - (count0[n] - count0[i + 1])),
        );
    }
    println!("{}", result);
}
