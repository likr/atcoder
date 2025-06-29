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
        a: [[Usize1]; m],
        b: [Usize1; n],
    }
    let mut edible = (0..m).collect::<HashSet<_>>();
    let mut dishes = vec![vec![]; n];
    for i in 0..m {
        for &aj in a[i].iter() {
            dishes[aj].push(i);
        }
    }
    let mut ans = vec![];
    for i in (0..n).rev() {
        ans.push(edible.len());
        for &aj in dishes[b[i]].iter() {
            edible.remove(&aj);
        }
    }
    ans.reverse();
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
