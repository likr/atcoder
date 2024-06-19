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
        s: Chars,
        t: Chars,
    }
    let mut index = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| (c, vec![]))
        .collect::<HashMap<char, Vec<usize>>>();
    for (i, &c) in s.iter().enumerate() {
        index.entry(c).or_insert(vec![]).push(i);
    }
    for &c in t.iter() {
        if index[&c].len() == 0 {
            println!("0");
            return;
        }
    }
    let mut ok = 0;
    let mut ng = INF;
    while ng - ok > 1 {
        let k = (ok + ng) / 2;
        let mut offset = 0;
        let mut count = 0;
        for &c in t.iter() {
            let m = index[&c].len();
            let x = index[&c].lower_bound(&offset);
            let a = m - x;
            if a >= k {
                offset = index[&c][x + k - 1] + 1;
            } else {
                let b = (k - a) % m;
                if b == 0 {
                    count += (k - a) / m;
                    offset = index[&c][m - 1] + 1;
                } else {
                    count += (k - a) / m + 1;
                    offset = index[&c][b - 1] + 1
                }
            }
        }
        if count < n {
            ok = k;
        } else {
            ng = k;
        }
    }
    println!("{}", ok);
}
