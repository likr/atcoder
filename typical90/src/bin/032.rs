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
        a: [[i64; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut ignore = HashSet::new();
    for &(xi, yi) in xy.iter() {
        ignore.insert((xi, yi));
        ignore.insert((yi, xi));
    }
    let mut result = INF as i64;
    let mut x = (0..n).collect::<Vec<_>>();
    loop {
        let mut ok = true;
        for i in 1..n {
            if ignore.contains(&(x[i - 1], x[i])) {
                ok = false;
            }
        }
        if ok {
            let mut s = 0;
            for i in 0..n {
                s += a[x[i]][i];
            }
            result = min(result, s);
        }
        if !x.next_permutation() {
            break;
        }
    }
    if result == INF as i64 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
