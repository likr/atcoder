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
    let s = s.into_iter().map(|si| si.into_iter().map(|c| c as u8 as usize - '0' as u8 as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut count = vec![vec![0; 10]; 10];
    for i in 0..n {
        for j in 0..10 {
            count[s[i][j]][j] += 1;
        }
    }
    let mut result = INF;
    for i in 0..10 {
        let k = (0..10).max_by_key(|&j| count[i][j]).unwrap();
        result = min(result, 10 * (count[i][k] - 1) + k);
    }
    println!("{}", result);
}
