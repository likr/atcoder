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
        mut a: [Usize1; n],
        b: [usize; n],
        c: [usize; n - 1],
    }

    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&k| a[k]);
    debug!(indices);

    let mut result = b.iter().sum::<usize>();
    for i in 1..n {
        if indices[i - 1] + 1 == indices[i] {
            result += c[i - 1];
        }
    }
    println!("{}", result);
}
