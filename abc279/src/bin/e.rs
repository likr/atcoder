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
        a: [Usize1; m],
    }
    let mut b = (0..n).collect::<Vec<_>>();
    let mut b_inv = b.clone();
    let mut right = (0..n).collect::<Vec<_>>();
    for i in (0..m).rev() {
        right.swap(a[i], a[i] + 1);
    }
    for i in 0..m {
        right.swap(a[i], a[i] + 1);
        debug!(b, b_inv, right);
        println!("{}", right[b_inv[0]] + 1);
        b_inv.swap(b[a[i]], b[a[i] + 1]);
        b.swap(a[i], a[i] + 1);
    }
}

/*
1 2 3 4 5
2 1 3 4 5
2 3 1 4 5
2 3 4 1 5
2 4 3 1 5



1 2 3 4 5
1 3 2 4 5
1 3 4 2 5
1 4 3 2 5
4 1 3 2 5
 */
