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
        xy: [(usize, usize); n],
        s: Chars,
    }
    let mut lines = HashMap::new();
    for i in 0..n {
        let (xi, yi) = xy[i];
        let si = s[i];
        lines.entry(yi).or_insert(vec![]).push((xi, si));
    }
    for line in lines.values_mut() {
        line.sort();
        let m = line.len();
        for i in 1..m {
            if line[i - 1].1 == 'R' && line[i].1 == 'L' {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
