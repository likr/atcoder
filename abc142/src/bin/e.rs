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
        m: usize,
        ac: [(usize, [Usize1]); m],
    }
    let keys = (0..m)
        .map(|i| {
            let mut key = 0;
            for &ci in ac[i].1.iter() {
                key |= 1 << ci;
            }
            key
        })
        .collect::<Vec<_>>();

    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for x in 0..1 << n {
        for i in 0..m {
            let ai = ac[i].0;
            let y = keys[i];
            dp[x | y] = min(dp[x | y], dp[x] + ai);
        }
    }
    if dp[(1 << n) - 1] == INF {
        println!("-1");
    } else {
        println!("{}", dp[(1 << n) - 1]);
    }
}
