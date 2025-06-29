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
    }
    let mut ans = vec![];
    ans.push(vec![1]);
    for i in 1..n {
        let mut row = vec![];
        row.push(1);
        for j in 1..i {
            row.push(ans[i - 1][j - 1] + ans[i - 1][j]);
        }
        row.push(1);
        ans.push(row);
    }
    for i in 0..n {
        println!(
            "{}",
            ans[i]
                .iter()
                .map(|aij| format!("{}", aij))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
