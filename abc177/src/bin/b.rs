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
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    let mut ans = m;
    for i in 0..n {
        if i + m > n {
            break;
        }
        let mut count = m;
        for j in 0..m {
            if s[i + j] == t[j] {
                count -= 1;
            }
        }
        ans = min(ans, count);
    }
    println!("{}", ans);
}
