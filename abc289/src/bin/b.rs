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
        a: [usize; m],
    }
    let mut b = vec![vec![]];
    for i in 1..m {
        let k = b.len();
        b[k - 1].push(a[i - 1]);
        if a[i - 1] + 1 != a[i] {
            b.push(vec![]);
        }
    }
    let mut p = (1..=n).collect::<Vec<_>>();
    if m > 0 {
        let k = b.len();
        b[k - 1].push(a[m - 1]);
        for bi in b.iter() {
            let start = bi[0] - 1;
            let stop = bi[bi.len() - 1];
            let k = stop - start + 1;
            for j in 0..k / 2 {
                p.swap(start + j, stop - j);
            }
        }
    }

    println!(
        "{}",
        p.iter()
            .map(|pi| format!("{}", pi))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
