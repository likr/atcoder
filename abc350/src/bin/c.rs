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
        mut a: [Usize1; n],
    }
    let mut index = vec![0; n];
    for i in 0..n {
        index[a[i]] = i;
    }
    let mut ans = vec![];
    for i in 0..n {
        if a[i] != i {
            let ai = a[i];
            ans.push((i, index[i]));
            a.swap(i, index[i]);
            index.swap(i, ai);
        }
    }
    println!("{}", ans.len());
    for &(i, j) in ans.iter() {
        println!("{} {}", i + 1, j + 1);
    }
}
