use petgraph::unionfind::UnionFind;
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
    let mut uf = UnionFind::new(200001);
    let mut result = 0;
    for i in 0..n / 2 {
        let x = a[i];
        let y = a[n - 1 - i];
        if uf.find(x) != uf.find(y) {
            uf.union(x, y);
            result += 1;
        }
    }
    println!("{}", result);
}
