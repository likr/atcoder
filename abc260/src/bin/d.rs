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
        k: usize,
        p: [Usize1; n],
    }
    let mut list = BTreeSet::new();
    let mut prev = vec![None; n];
    let mut count = vec![0; n];
    let mut ans = vec![INF; n];
    for i in 0..n {
        if let Some(&v) = list.range(p[i]..).nth(0) {
            prev[p[i]] = Some(v);
            count[p[i]] = count[v] + 1;
            list.remove(&v);
        } else {
            count[p[i]] = 1;
        }
        list.insert(p[i]);
        if count[p[i]] == k {
            list.remove(&p[i]);
            ans[p[i]] = i + 1;
            let mut u = p[i];
            while let Some(v) = prev[u] {
                u = v;
                ans[v] = i + 1;
            }
        }
    }
    for i in 0..n {
        if ans[i] == INF {
            println!("-1");
        } else {
            println!("{}", ans[i]);
        }
    }
}
