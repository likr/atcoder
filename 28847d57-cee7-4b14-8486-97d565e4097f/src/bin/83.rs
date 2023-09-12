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
    let chars = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let mut s = vec![vec![]; n];
    s[0].push(vec![0]);
    for j in 1..n {
        for k in 0..s[j - 1].len() {
            for i in 0..=s[j - 1][k].iter().max().unwrap() + 1 {
                let mut ti = s[j - 1][k].clone();
                ti.push(i);
                s[j].push(ti);
            }
        }
    }
    for si in s[n - 1].iter() {
        println!("{}", si.into_iter().map(|&i| chars[i]).collect::<String>());
    }
}
