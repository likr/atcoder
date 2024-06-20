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
    let mut count = HashMap::new();
    for &c in s.iter() {
        *count.entry(c).or_insert(0) += 1;
    }
    for (&c, &count) in count.iter() {
        if count == 1 {
            for i in 0..s.len() {
                if s[i] == c {
                    println!("{}", i + 1);
                }
            }
            return;
        }
    }
}
