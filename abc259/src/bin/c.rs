use itertools::*;
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
        s: Chars,
        t: Chars,
    }
    let s_count = s
        .iter()
        .group_by(|&&c| c)
        .into_iter()
        .map(|(c, cs)| (c, cs.count()))
        .collect::<Vec<_>>();
    let t_count = t
        .iter()
        .group_by(|&&c| c)
        .into_iter()
        .map(|(c, cs)| (c, cs.count()))
        .collect::<Vec<_>>();
    let n = s_count.len();
    if n != t_count.len() {
        println!("No");
        return;
    }
    for i in 0..n {
        if s_count[i].0 != t_count[i].0
            || s_count[i].1 == 1 && t_count[i].1 > 1
            || s_count[i].1 > t_count[i].1
        {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
