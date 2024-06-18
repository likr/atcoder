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
const INF: i64 = std::i64::MAX / 4;
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
        q: usize,
        k: i64,
        tx: [(usize, i64); q],
    }
    let mut xs = vec![-INF, INF];
    for i in 0..q {
        xs.push(tx[i].1);
    }
    xs.sort();
    xs.dedup();
    debug!(xs);
    let mut index = HashMap::new();
    for (i, &xi) in xs.iter().enumerate() {
        index.insert(xi, i);
    }
    let n = xs.len();
    let mut nodes = Segtree::<Additive<usize>>::new(n);
    nodes.set(0, 1);
    nodes.set(n - 1, 1);
    let mut adj = Segtree::<Additive<usize>>::new(n);
    for &(ti, xi) in tx.iter() {
        let j = index[&xi];
        if ti == 1 {
            if nodes.get(j) == 0 {
                let r = nodes.max_right(j, |&s| s == 0);
                if xs[r] - xs[j] <= k {
                    adj.set(j, 1);
                }
                let l = nodes.min_left(j, |&s| s == 0) - 1;
                if xs[j] - xs[l] <= k {
                    adj.set(l, 1);
                }
                nodes.set(j, 1);
            } else {
                adj.set(j, 0);
                let l = nodes.min_left(j, |&s| s == 0) - 1;
                if xs[j] - xs[l] <= k {
                    adj.set(l, 0);
                }
                nodes.set(j, 0);
                nodes.set(l, 0);
                let r = nodes.max_right(l, |&s| s == 0);
                nodes.set(l, 1);
                if j < r && xs[r] - xs[l] <= k {
                    adj.set(l, 1);
                }
            }
        } else {
            let tmp = adj.get(j);
            adj.set(j, 1);
            let mut ng = 0;
            let mut ok = j;
            while ok - ng > 1 {
                let m = (ok + ng) / 2;
                if nodes.prod(m..=j) == adj.prod(m..=j) {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let l = ok;
            adj.set(j, tmp);
            let mut ok = l - 1;
            let mut ng = n - 1;
            while ng - ok > 1 {
                let m = (ok + ng) / 2;
                if nodes.prod(l..=m) == adj.prod(l..=m) {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            let r = ng;
            println!("{}", nodes.prod(l..=r));
        }
    }
}
