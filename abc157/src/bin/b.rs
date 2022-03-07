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
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }
    let set = b.iter().collect::<HashSet<_>>();
    let indices = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    for is in indices.iter() {
        if is.iter().all(|&k| {
            let i = k / 3;
            let j = k % 3;
            set.contains(&a[i][j])
        }) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
