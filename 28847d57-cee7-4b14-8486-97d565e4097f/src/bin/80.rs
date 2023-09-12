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
        k: usize,
        p: [usize; n],
    }
    let mut ans = vec![INF; n + 1];
    let mut dsu = Dsu::new(n + 1);
    let mut board = BTreeSet::new();
    for i in 0..n {
        if let Some(&x) = board.range(p[i]..).nth(0) {
            dsu.merge(p[i], x);
            board.remove(&x);
        }
        board.insert(p[i]);
        if dsu.size(p[i]) == k {
            ans[dsu.leader(p[i])] = i + 1;
            board.remove(&p[i]);
        }
    }
    for g in dsu.groups().iter() {
        for &i in g.iter() {
            ans[i] = ans[dsu.leader(i)];
        }
    }
    for i in 1..=n {
        if ans[i] == INF {
            println!("-1");
        } else {
            println!("{}", ans[i]);
        }
    }
}
