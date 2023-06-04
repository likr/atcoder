use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        mut s: [Chars; n],
    }
    s.sort();
    loop {
        let mut ok = true;
        for i in 1..n {
            let mut count = 0;
            for j in 0..m {
                if s[i - 1][j] != s[i][j] {
                    count += 1;
                }
            }
            if count != 1 {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        if !s.next_permutation() {
            break;
        }
    }
    println!("No");
}
