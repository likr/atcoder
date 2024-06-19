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
        c: [usize; n],
    }
    let mut cost0 = vec![0; n];
    let mut cost1 = vec![0; n];
    for i in 0..n {
        if (i % 2 == 0 && s[i] == '0') || (i % 2 == 1 && s[i] == '1') {
            cost1[i] = c[i];
        } else {
            cost0[i] = c[i];
        }
    }
    for i in 1..n {
        cost0[i] += cost0[i - 1];
        cost1[i] += cost1[i - 1];
    }
    let mut ans = INF;
    for i in 1..n {
        ans = min(ans, cost0[i - 1] + cost1[n - 1] - cost1[i - 1]);
        ans = min(ans, cost1[i - 1] + cost0[n - 1] - cost0[i - 1]);
    }
    println!("{}", ans);
}
