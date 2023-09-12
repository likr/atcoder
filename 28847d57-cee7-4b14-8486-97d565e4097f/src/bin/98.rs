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
        mut uvw: [(Usize1, Usize1, usize); n - 1],
    }
    uvw.sort_by_key(|&(_, _, w)| w);
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for &(u, v, w) in uvw.iter() {
        ans += w * dsu.size(u) * dsu.size(v);
        dsu.merge(u, v);
    }
    println!("{}", ans);
}
