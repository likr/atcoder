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
        x: usize,
        y: usize,
        pt: [(usize, usize); n - 1 ],
        q: [usize],
    }
    let mut ans = vec![0; 840];
    for s in 0..840 {
        let mut t = s + x;
        for i in 1..n {
            let (pi, ti) = pt[i - 1];
            t = (t + pi - 1) / pi * pi + ti;
        }
        t += y;
        ans[s] = t;
    }
    for qi in q.iter() {
        println!("{}", ans[qi % 840] + 840 * (qi / 840))
    }
}
