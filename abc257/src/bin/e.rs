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
        mut n: usize,
        c: [usize; 9],
    }
    let mut indices = (0..9).collect::<Vec<_>>();
    indices.sort_by_key(|&i| (c[i], Reverse(i)));
    debug!(indices);
    let min_c = c[indices[0]];
    let m = n / min_c;
    debug!(m);
    let mut x = vec![indices[0]; m];
    n -= min_c * m;
    for i in 0..m {
        let mut xi = indices[0];
        for j in 1..9 {
            if (c[indices[j]] - c[indices[0]]) <= n && indices[j] > xi {
                xi = indices[j];
            }
        }
        x[i] = xi;
        n -= c[xi] - c[indices[0]];
    }
    let mut result = vec![];
    for i in 0..m {
        result.push(format!("{}", x[i] + 1));
    }
    println!("{}", result.join(""));
}
