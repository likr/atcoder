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
    let mut events = vec![];
    for i in 0..n {
        events.push((a[i], b[i], 0, i));
    }
    for j in 0..m {
        events.push((c[j], d[j], 1, j));
    }
    events.sort();
    let mut set = BTreeSet::new();
    for &(_, yi, t, k) in events.iter() {
        if t == 0 {
            set.insert((yi, k));
        } else {
            if let Some(&r) = set.range(..=(yi, INF)).last() {
                set.remove(&r);
            }
        }
    }
    if set.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
