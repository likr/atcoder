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
        s: [Chars; n],
    }
    let mut result = 0;
    for x in 1..1 << n {
        let mut count = HashMap::new();
        for i in 0..n {
            if x & 1 << i > 0 {
                for &c in s[i].iter() {
                    *count.entry(c).or_insert(0) += 1;
                }
            }
        }
        result = max(result, count.values().filter(|&&v| v == k).count());
    }
    println!("{}", result);
}
