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
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }
    let mut a_index = HashMap::new();
    let mut b_index = HashMap::new();
    for i in 0..n {
        if !a_index.contains_key(&a[i]) {
            a_index.insert(a[i], i);
        }
        if !b_index.contains_key(&b[i]) {
            b_index.insert(b[i], i);
        }
    }
    let mut a_tree = Segtree::<Max<_>>::new(n);
    let mut b_tree = Segtree::<Max<_>>::new(n);
    for i in 0..n {
        if let Some(&k) = b_index.get(&a[i]) {
            a_tree.set(i, k);
        } else {
            a_tree.set(i, INF);
        }
        if let Some(&k) = a_index.get(&b[i]) {
            b_tree.set(i, k);
        } else {
            b_tree.set(i, INF);
        }
    }
    for &(xi, yi) in xy.iter() {
        if a_tree.prod(0..xi + 1) <= yi && b_tree.prod(0..yi + 1) <= xi {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
