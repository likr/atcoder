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
        k: usize,
        mut s: Chars,
    }
    s.push('x');
    let mut count = 1;
    let mut segs = vec![];
    for i in 0..n {
        if s[i] != s[i + 1] {
            segs.push(count);
            count = 0;
        }
        count += 1;
    }
    debug!(segs);

    let m = segs.len();
    let mut acc = vec![0; m + 1];
    for i in 0..m {
        acc[i + 1] = acc[i] + segs[i];
    }

    let l = 2 * k + 1;
    if l >= m {
        println!("{}", n - 1);
        return;
    }

    debug!(acc);
    let x = (l..=m).max_by_key(|&i| acc[i] - acc[i - l]).unwrap();
    debug!(x);
    let mut result = acc[x] - acc[x - l] - 1;
    for i in 0..m {
        if i < x - l || x <= i {
            debug!(i);
            result += segs[i] - 1;
        }
    }
    println!("{}", result);
}
