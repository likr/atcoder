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
            si.into_iter()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut ans = INF;
    for i in 0..10 {
        let mut count = vec![0; 10];
        for j in 0..n {
            for k in 0..10 {
                if s[j][k] == i {
                    count[k] += 1;
                }
            }
        }
        let k_max = (0..10).max_by_key(|&k| (count[k], k)).unwrap();
        ans = min(ans, 10 * (count[k_max] - 1) + k_max);
    }
    println!("{}", ans);
}
