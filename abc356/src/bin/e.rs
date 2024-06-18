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
        mut a: [usize; n],
    }
    let m = *a.iter().max().unwrap();
    let mut tree = Segtree::<Additive<usize>>::new(m + 1);
    let mut count = vec![0; m + 1];
    for i in 0..n {
        tree.set(a[i], tree.get(a[i]) + 1);
        count[a[i]] += 1;
    }
    a.sort();
    a.dedup();
    let mut ans = 0;
    for &ai in a.iter() {
        ans += count[ai] * (count[ai] - 1) / 2;
        ans += count[ai] * tree.prod(ai + 1..min(2 * ai, m + 1));
        for j in 2.. {
            if ai * j > m {
                break;
            }
            ans += count[ai] * j * tree.prod(ai * j..min(ai * (j + 1), m + 1));
        }
    }
    println!("{}", ans);
}
