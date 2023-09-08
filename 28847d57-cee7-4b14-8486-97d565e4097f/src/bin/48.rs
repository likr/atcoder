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
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut pairs = HashSet::new();
    for &(xi, yi) in xy.iter() {
        pairs.insert((xi, yi));
        pairs.insert((yi, xi));
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    let mut ans = INF;
    loop {
        let mut ok = true;
        for i in 1..n {
            if pairs.contains(&(indices[i - 1], indices[i])) {
                ok = false;
            }
        }
        if ok {
            let mut s = 0;
            for j in 0..n {
                s += a[indices[j]][j];
            }
            ans = min(ans, s);
        }
        if !indices.next_permutation() {
            break;
        }
    }
    if ans == INF {
        println!("-1")
    } else {
        println!("{}", ans);
    }
}
