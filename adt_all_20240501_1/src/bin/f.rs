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
        s: String,
    }
    let mut ans = 0;
    for si in s.split('0') {
        let mut count1 = 0;
        let mut count2 = 0;
        for c in si.chars() {
            if c == '1' {
                count1 += 1;
            }
            if c == '2' {
                count2 += 1;
            }
        }
        ans = max(ans, if count1 > m { count1 - m } else { 0 } + count2);
    }
    println!("{}", ans);
}
