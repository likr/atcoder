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
        s: [Chars; n],
    }
    let s = s
        .into_iter()
        .map(|si| {
            (
                (si[0] as u8 - 'a' as u8) as usize,
                (si[si.len() - 1] as u8 - 'a' as u8) as usize,
            )
        })
        .collect::<Vec<_>>();
    let mut x = (0..1 << n).collect::<Vec<_>>();
    x.sort_by_key(|&xi| (0..n).filter(|&i| xi & 1 << i > 0).count());
    let mut dp = vec![vec![false; 26]; x.len()];
    for &xi in x.iter() {
        for j in 0..26 {
            for i in 0..n {
                if s[i].0 == j && xi & 1 << i > 0 && !dp[xi ^ 1 << i][s[i].1] {
                    dp[xi][j] = true;
                }
            }
        }
    }
    if (0..26).any(|j| dp[(1 << n) - 1][j]) {
        println!("First");
    } else {
        println!("Second");
    }
}
