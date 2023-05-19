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
        m: usize,
        b: [usize; m],
        x: usize,
    }
    let b = b.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 0..x {
        if dp[i] && !b.contains(&i) {
            for j in 0..n {
                if i + a[j] <= x {
                    dp[i + a[j]] = true;
                }
            }
        }
    }
    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
