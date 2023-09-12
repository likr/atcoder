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
        s: Chars,
    }
    let chars = "0123456789".chars().collect::<Vec<_>>();
    let mut ans = 0;
    for &c1 in chars.iter() {
        for &c2 in chars.iter() {
            for &c3 in chars.iter() {
                let t = vec![c1, c2, c3];
                let mut j = 0;
                for i in 0..n {
                    if s[i] == t[j] {
                        j += 1;
                    }
                    if j == 3 {
                        break;
                    }
                }
                if j == 3 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
