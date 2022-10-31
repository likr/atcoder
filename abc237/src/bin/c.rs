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
    }
    let n = s.len();
    let head = s.iter().take_while(|&&c| c == 'a').count();
    let tail = s.iter().rev().take_while(|&&c| c == 'a').count();
    if tail == n {
        println!("Yes");
        return;
    }
    if tail > head {
        for _ in 0..(tail - head) {
            s.pop();
        }
    }
    let m = s.len();
    if (0..m).all(|i| s[i] == s[m - i - 1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
