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
        mut a: [usize; n],
        mut b: [usize; m],
    }
    b.sort();
    let mut a_set = BTreeSet::new();
    for i in 0..n {
        a_set.insert((a[i], i));
    }
    let mut ans = 0;
    for j in 0..m {
        if let Some(&(c, i)) = a_set.range((b[j], 0)..).nth(0) {
            ans += c;
            a_set.remove(&(c, i));
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
