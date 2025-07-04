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
    }
    let mut indices = HashMap::new();
    for i in 0..n {
        indices.entry(a[i]).or_insert(vec![]).push(i);
    }
    let mut values = indices.keys().map(|&v| v).collect::<Vec<_>>();
    values.sort();
    values.reverse();
    let mut s = 0;
    let mut ans = vec![0; n];
    for &v in values.iter() {
        for &i in indices[&v].iter() {
            ans[i] = s;
        }
        s += v * indices[&v].len();
    }
    println!(
        "{}",
        ans.iter()
            .map(|&v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
