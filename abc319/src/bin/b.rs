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
    let mut ans = vec!['-'; n + 1];
    for i in 0..=n {
        for j in 1..=9 {
            if n % j == 0 && i % (n / j) == 0 {
                ans[i] = ('0' as usize + j) as u8 as char;
                break;
            }
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
