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
    }
    let mut ans = 0;
    let mut max_count = 0;
    for k in 2..=1000 {
        let mut count = 0;
        for i in 0..n {
            if a[i] % k == 0 {
                count += 1;
            }
        }
        if count >= max_count {
            ans = k;
            max_count = count;
        }
    }
    println!("{}", ans);
}
