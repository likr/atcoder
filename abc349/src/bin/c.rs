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
        mut s: Chars,
        t: Chars,
    }
    let n = s.len();
    for i in 0..n {
        s[i] = s[i].to_ascii_uppercase();
    }
    s.push('X');
    let mut i = 0;
    for &tj in t.iter() {
        while i <= n && s[i] != tj {
            i += 1;
        }
        if i > n || s[i] != tj {
            println!("No");
            return;
        }
        i += 1;
    }
    println!("Yes");
}
