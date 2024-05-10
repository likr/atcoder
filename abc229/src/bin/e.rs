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
        ab: [(Usize1, Usize1); m],
    }
    let mut edges = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        edges[ai].push(bi);
    }
    let mut count = 0;
    let mut ans = vec![];
    let mut dsu = Dsu::new(n);
    for u in (0..n).rev() {
        ans.push(count);
        count += 1;
        for &v in edges[u].iter() {
            if !dsu.same(u, v) {
                count -= 1;
                dsu.merge(u, v);
            }
        }
    }
    ans.reverse();
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
