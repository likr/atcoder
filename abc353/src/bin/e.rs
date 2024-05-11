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
        mut s: [Chars; n],
    }
    s.sort();

    let mut adj = vec![0; n - 1];
    for i in 1..n {
        for j in 0.. {
            if j >= s[i - 1].len() || j >= s[i].len() || s[i - 1][j] != s[i][j] {
                break;
            }
            adj[i - 1] += 1;
        }
    }

    let mut t = Segtree::<Min<usize>>::new(n);
    for i in 1..n {
        t.set(i, adj[i - 1]);
    }
    let mut ans = 0;
    for j in 1..n {
        for k in 1..=adj[j - 1] {
            let mut ng = 0;
            let mut ok = j;
            while ng + 1 < ok {
                let m = (ok + ng) / 2;
                if t.prod(m..=j) < k {
                    ng = m;
                } else {
                    ok = m;
                }
            }
            ans += j + 1 - ok;
        }
    }
    println!("{}", ans);
}
