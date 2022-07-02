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
        m: usize,
        t: usize,
        mut ab: [(usize, usize); m],
    }
    ab.reverse();
    ab.push((0, 0));
    ab.reverse();
    ab.push((t, t));
    let m = ab.len();
    let mut x = n;
    for i in 1..m {
        let s = ab[i].0 - ab[i - 1].1;
        let t = ab[i].1 - ab[i].0;
        if x <= s {
            println!("No");
            return;
        }
        x -= s;
        x = min(n, x + t);
    }
    println!("Yes");
}
