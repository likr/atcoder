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
        n: usize,
        p: [usize; n],
    }
    let mut indices = vec![0; n + 1];
    for i in 0..n {
        indices[p[i]] = i as i64;
    }
    let mut visited = BTreeMap::new();
    visited.insert(-2, INF);
    visited.insert(-1, INF);
    visited.insert(n as i64, INF);
    visited.insert(n as i64 + 1, INF);
    visited.insert(indices[n], n as i64);
    let mut result = 0;
    for x in (1..n).rev() {
        let m = indices[x];
        let mut left = visited.range(..m).rev();
        let l1 = *left.next().unwrap().0;
        let l2 = *left.next().unwrap().0;
        let mut right = visited.range(m..);
        let r1 = *right.next().unwrap().0;
        let r2 = *right.next().unwrap().0;
        debug!(l2, l1, m, r1, r2);
        if 0 <= r1 && r1 < n as i64 {
            result += x as i64 * (m - l1) * (r2 - r1);
        }
        if 0 <= l1 && l1 < n as i64 {
            result += x as i64 * (l1 - l2) * (r1 - m);
        }
        visited.insert(indices[x], x as i64);
    }
    println!("{}", result);
}
