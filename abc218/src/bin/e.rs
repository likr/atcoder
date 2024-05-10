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
        m: usize,
        mut abc: [(Usize1, Usize1, i64); m],
    }
    abc.sort_by_key(|&(_, _, c)| c);
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for &(u, v, c) in abc.iter() {
        if c <= 0 {
            dsu.merge(u, v);
        } else if dsu.same(u, v) {
            ans += c;
        } else {
            dsu.merge(u, v);
        }
    }
    println!("{}", ans);
}
