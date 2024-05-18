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
        ac: [(usize, usize); n],
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| (Reverse(ac[i].0), ac[i].1));
    let mut ans = vec![indices[0]];
    for i in 1..n {
        let (a0, c0) = ac[ans[ans.len() - 1]];
        let (ai, ci) = ac[indices[i]];
        if a0 <= ai || c0 >= ci {
            ans.push(indices[i]);
        }
    }
    ans.sort();
    let ans = ans.iter().map(|i| format!("{}", i + 1)).collect::<Vec<_>>();
    println!("{}", ans.len());
    println!("{}", ans.join(" "));
}
