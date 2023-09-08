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
    indices.sort_by_key(|&i| w[i]);
    let one_total = s.iter().filter(|&&c| c == '1').count();
    let zero_total = n - one_total;
    let mut result = one_total;
    let mut one_count = 0;
    let mut zero_count = 0;
    for i in 0..n {
        if s[indices[i]] == '1' {
            one_count += 1;
        } else {
            zero_count += 1;
        }
        if i + 1 == n || w[indices[i]] != w[indices[i + 1]] {
            result = max(result, n - one_count - (zero_total - zero_count));
        }
    }
    println!("{}", result);
}
