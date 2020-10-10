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
const M: usize = 998244353;

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
        k: usize,
        lr: [(usize, usize); k],
    }
    let mut c = 0;
    let mut insert = BinaryHeap::new();
    let mut remove = BinaryHeap::new();
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 0..k {
        let (li, ri) = lr[i];
        insert.push(Reverse((1 + li, 1)));
        remove.push(Reverse((1 + ri, 1)));
    }
    for t in 2..=n {
        while let Some(&Reverse((t2, s))) = insert.peek() {
            if t2 > t {
                break;
            }
            c = (c + s) % M;
            insert.pop();
        }
        debug!(t, c);
        dp[t] = c;
        while let Some(&Reverse((t2, s))) = remove.peek() {
            if t2 > t {
                break;
            }
            c = (c + M - s) % M;
            remove.pop();
        }
        for i in 0..k {
            let (li, ri) = lr[i];
            insert.push(Reverse((t + li, dp[t])));
            remove.push(Reverse((t + ri, dp[t])));
        }
    }
    debug!(dp);
    println!("{}", dp[n]);
}
