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
    }
    let mut nums = HashSet::new();
    let mut result = n;
    for a in 2.. {
        if a * a > n {
            break;
        }
        let mut s = a;
        loop {
            s *= a;
            if s > n {
                break;
            }
            if !nums.contains(&s) {
                result -= 1;
            }
            nums.insert(s);
        }
    }
    println!("{}", result);
}
