use ac_library::*;
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
        a: [usize; n],
    }
    let mut acc = Segtree::<Additive<usize>>::new(n);
    let mut count = Segtree::<Additive<usize>>::new(n);
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| a[i]);
    let mut ans = 0;
    for &i in indices.iter() {
        acc.set(i, a[i]);
        count.set(i, 1);
        ans += count.prod(0..i) * a[i] - acc.prod(0..i);
    }
    println!("{}", ans);
}
