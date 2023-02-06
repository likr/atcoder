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
        st: [(String, String); n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        let (si, ti) = st[i].clone();
        *count.entry(si).or_insert(0) += 1;
        *count.entry(ti).or_insert(0) += 1;
    }
    for i in 0..n {
        let (si, ti) = st[i].clone();
        if si == ti {
            if count[&si] > 2 {
                println!("No");
                return;
            }
        } else {
            if count[&si] > 1 && count[&ti] > 1 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
