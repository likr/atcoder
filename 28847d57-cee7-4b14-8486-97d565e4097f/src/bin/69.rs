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
        uv: [(Usize1, Usize1); m],
        k: usize,
        xy: [(Usize1, Usize1); k],
        q: usize,
        pq: [(Usize1, Usize1); q],
    }
    let mut dsu = Dsu::new(n);
    for &(ui, vi) in uv.iter() {
        dsu.merge(ui, vi);
    }
    let mut pairs = HashSet::new();
    for &(xi, yi) in xy.iter() {
        pairs.insert((dsu.leader(xi), dsu.leader(yi)));
        pairs.insert((dsu.leader(yi), dsu.leader(xi)));
    }
    for &(pi, qi) in pq.iter() {
        if pairs.contains(&(dsu.leader(pi), dsu.leader(qi))) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
