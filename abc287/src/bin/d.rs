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
    let mut match_count = 0;
    for i in 0..m {
        if s[n - m + i] == '?' || t[i] == '?' || s[n - m + i] == t[i] {
            match_count += 1;
        }
    }
    if match_count == m {
        println!("Yes");
    } else {
        println!("No");
    }
    for i in 0..m {
        if s[n - m + i] == '?' || t[i] == '?' || s[n - m + i] == t[i] {
            if s[i] != '?' && t[i] != '?' && s[i] != t[i] {
                match_count -= 1;
            }
        } else {
            if s[i] == '?' || t[i] == '?' || s[i] == t[i] {
                match_count += 1;
            }
        }
        if match_count == m {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
