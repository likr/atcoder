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
        s: Chars,
    }
    let n = s.len();
    let s = s
        .into_iter()
        .map(|si| (si as u8 - '0' as u8) as usize)
        .collect::<Vec<_>>();
    let mut count = vec![vec![0usize; n]; 10];
    for i in 0..n {
        count[s[i]][i] += 1;
    }
    for j in 0..10 {
        for i in 1..n {
            count[j][i] += count[j][i - 1];
        }
        count[j].reverse();
        count[j].push(0);
        count[j].reverse();
    }
    debug!(count);
    let mut patterns = HashMap::new();
    for i in 0..=n {
        let mut x = 0;
        for j in 0..10 {
            if count[j][i] % 2 == 0 {
                x |= 1 << j;
            }
        }
        *patterns.entry(x).or_insert(0) += 1;
    }
    debug!(patterns);
    let mut result = 0usize;
    for &v in patterns.values() {
        result += v * (v - 1) / 2;
    }
    println!("{}", result);
}
