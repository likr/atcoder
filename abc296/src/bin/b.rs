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
        mut s: [Chars; 8],
    }
    s.reverse();
    let col = "abcdefgh".chars().collect::<Vec<_>>();
    let row = "12345678".chars().collect::<Vec<_>>();
    let mut result = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                result.push(format!("{}{}", col[j], row[i]));
            }
        }
    }
    println!("{}", result.join(" "));
}
