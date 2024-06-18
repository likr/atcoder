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
        t: usize,
        a: [Usize1; t],
    }
    let mut b = vec![vec![INF; n]; n];
    for i in 0..t {
        b[a[i] / n][a[i] % n] = i + 1;
    }
    let mut ans = INF;
    for i in 0..n {
        ans = min(ans, (0..n).map(|j| b[i][j]).max().unwrap());
        ans = min(ans, (0..n).map(|j| b[j][i]).max().unwrap());
    }
    ans = min(ans, (0..n).map(|i| b[i][i]).max().unwrap());
    ans = min(ans, (0..n).map(|i| b[i][n - 1 - i]).max().unwrap());
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
