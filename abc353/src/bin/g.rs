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
        c: i64,
        m: usize,
        tp: [(Usize1, i64); m],
    }
    let mut dp1 = Segtree::<Max<i64>>::new(n);
    let mut dp2 = Segtree::<Max<i64>>::new(n);
    for i in 0..n {
        dp1.set(i, -c * i as i64 + c * i as i64);
        dp2.set(i, -c * i as i64 - c * i as i64);
    }
    for &(ti, pi) in tp.iter() {
        let q = max(
            dp1.prod(0..ti) - c * ti as i64,
            dp2.prod(ti..n) + c * ti as i64,
        ) + pi;
        dp1.set(ti, q + c * ti as i64);
        dp2.set(ti, q - c * ti as i64);
    }
    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, dp1.get(i) - c * i as i64);
        ans = max(ans, dp2.get(i) + c * i as i64);
    }
    println!("{}", ans);
}
