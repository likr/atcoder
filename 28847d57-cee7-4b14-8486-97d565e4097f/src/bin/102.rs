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
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }
    let mut xy = vec![];
    for i in 0..n {
        xy.push((a[i], b[i], 0, i));
    }
    for i in 0..m {
        xy.push((c[i], d[i], 1, 0));
    }
    xy.sort();
    let mut chocolate = BTreeSet::new();
    for &(_, yi, ti, i) in xy.iter() {
        if ti == 0 {
            chocolate.insert((yi, i));
        } else {
            if let Some(&item) = chocolate.range(..=(yi, INF)).rev().nth(0) {
                chocolate.remove(&item);
            }
        }
    }
    if chocolate.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
