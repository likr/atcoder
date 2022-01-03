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
        q: usize,
        tx: [(usize, usize); q],
    }
    let mut head = vec![];
    let mut tail = vec![];
    for i in 0..q {
        let (ti, xi) = tx[i];
        if ti == 1 {
            head.push(xi);
        } else if ti == 2 {
            tail.push(xi);
        } else {
            let k = xi - 1;
            if k < head.len() {
                println!("{}", head[head.len() - 1 - k]);
            } else {
                println!("{}", tail[k - head.len()]);
            }
        }
    }
}
