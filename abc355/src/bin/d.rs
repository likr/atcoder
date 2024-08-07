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
        lr: [(usize, usize); n],
    }
    let mut events = vec![];
    for &(li, ri) in lr.iter() {
        events.push((li, 0));
        events.push((ri, 1));
    }
    events.sort();
    let mut count = 0;
    let mut ans = 0usize;
    for &(_, f) in events.iter() {
        if f == 0 {
            count += 1;
        } else {
            count -= 1;
            ans += count;
        }
    }
    println!("{}", ans);
}
