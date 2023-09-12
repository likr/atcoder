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
        mut a: [usize; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }
    a.sort();
    a.dedup();
    let m = *a.iter().max().unwrap() + 1;
    let mut f = vec![true; m];
    let mut ans = 0;
    for i in 0..a.len() {
        if f[a[i]] {
            if count[&a[i]] == 1 {
                ans += 1;
            }
            for j in (2 * a[i]..m).step_by(a[i]) {
                f[j] = false;
            }
        }
    }
    println!("{}", ans);
}
