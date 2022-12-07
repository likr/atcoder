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
        s: [Chars; n],
    }
    let first_letter = "HDCS".chars().collect::<Vec<_>>();
    let second_letter = "A23456789TJQK".chars().collect::<Vec<_>>();
    if s.iter().all(|si| {
        first_letter.iter().any(|&c| c == si[0]) && second_letter.iter().any(|&c| c == si[1])
    }) && s.iter().collect::<HashSet<_>>().len() == n
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
