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
        q: usize,
        abc: [(Usize1, Usize1, usize); m],
        uvw: [(Usize1, Usize1, usize); q],
    }
    let mut edges = vec![];
    for &(u, v, w) in abc.iter() {
        edges.push((u, v, w, None));
    }
    for (i, &(u, v, w)) in uvw.iter().enumerate() {
        edges.push((u, v, w, Some(i)));
    }
    edges.sort_by_key(|&(_, _, w, _)| w);
    let mut ans = vec![false; q];
    let mut dsu = Dsu::new(n);
    for &(u, v, _, i) in edges.iter() {
        if !dsu.same(u, v) {
            if let Some(i) = i {
                ans[i] = true;
            } else {
                dsu.merge(u, v);
            }
        }
    }
    for i in 0..q {
        if ans[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
