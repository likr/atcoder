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
        m: usize,
        mut ab: [(Usize1, Usize1); n],
    }
    ab.sort();
    let mut set = BTreeMap::new();
    for i in 0..n {
        *set.entry(ab[i].0).or_insert(0) += 1;
    }
    let mut bit = FenwickTree::new(n, 0);
    let mut ans = (1..=m).rev().collect::<Vec<_>>();
    let mut j = 0;
    let mut k = 0;
    for i in 0..m {
        while k < n && ab[k].0 == i {
            *set.entry(ab[k].1).or_insert(0) += 1;
            *set.entry(ab[k].0).or_insert(1) -= 1;
            if set[&ab[k].0] == 0 {
                set.remove(&ab[k].0);
            }
            k += 1;
        }
        while j < m && set.iter().nth(0).unwrap() > j 
    }

    println!(
        "{}",
        ans.iter()
            .map(|&v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
