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
        k: usize,
        s: Chars,
    }
    let mut count = vec![0];
    for i in 0..n {
        if (s[i] == '0' && count.len() % 2 == 1) || (s[i] == '1' && count.len() % 2 == 0) {
            count.push(0);
        }
        *count.last_mut().unwrap() += 1;
    }
    if s[n - 1] == '0' {
        count.push(0);
    }
    let m = count.len();

    let mut acc = vec![0; m + 1];
    for i in 0..m {
        acc[i + 1] = acc[i] + count[i];
    }

    if m / 2 <= k {
        println!("{}", acc.last().unwrap());
        return;
    }

    let mut result = 0;
    for i in (2 * k..m).step_by(2) {
        result = max(result, acc[i + 1] - acc[i - 2 * k]);
    }
    println!("{}", result);
}
