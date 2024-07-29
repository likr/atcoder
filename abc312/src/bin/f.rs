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

fn lower_bound(count: &FenwickTree<usize>, n: usize, m: usize) -> usize {
    let mut ok = 0;
    let mut ng = n;
    while ng - ok > 1 {
        let k = (ok + ng) / 2;
        if count.accum(k) >= m {
            ng = k;
        } else {
            ok = k;
        }
    }
    ok
}

fn main() {
    input! {
        n: usize,
        m: usize,
        tx: [(usize, usize); n],
    }
    let mut items = vec![];
    let mut c = vec![];
    for &(ti, xi) in tx.iter() {
        if ti == 2 {
            c.push(xi);
        } else {
            items.push((xi, ti));
        }
    }
    items.sort();
    items.reverse();
    c.sort();
    let mut b = vec![];
    let n = items.len();
    let mut acc = FenwickTree::new(n, 0);
    let mut count = FenwickTree::new(n, 0);
    for i in 0..n {
        if items[i].1 == 0 {
            acc.add(i, items[i].0);
            count.add(i, 1);
        } else {
            b.push((items[i].0, i));
        }
    }
    b.sort();
    let k = lower_bound(&count, n, m);
    let mut ans = acc.sum(0..min(n, k + 1));
    let mut c_count = 0;
    while let Some(x) = c.pop() {
        for _ in 0..min(x, b.len()) {
            let (y, j) = b.pop().unwrap();
            acc.add(j, y);
            count.add(j, 1);
        }
        c_count += 1;
        if m < c_count {
            break;
        }
        let k = lower_bound(&count, n, m - c_count);
        ans = max(ans, acc.sum(0..min(n, k + 1)));
    }
    println!("{}", ans);
}
