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
        q: usize,
        x: [Usize1; q],
    }
    let mut set = HashSet::new();
    let mut acc = vec![0; q + 1];
    let mut range = vec![vec![]; n];
    for i in 0..q {
        if set.contains(&x[i]) {
            set.remove(&x[i]);
        } else {
            set.insert(x[i]);
        }
        range[x[i]].push(i);
        acc[i + 1] = set.len();
    }
    for i in 0..q {
        acc[i + 1] += acc[i];
    }
    let mut ans = vec![];
    for i in 0..n {
        if range[i].len() % 2 == 1 {
            range[i].push(q);
        }
        let mut s = 0;
        for j in (0..range[i].len()).step_by(2) {
            s += acc[range[i][j + 1]] - acc[range[i][j]];
        }
        ans.push(format!("{}", s));
    }
    println!("{}", ans.join(" "));
}
