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
        k: usize,
        a: [usize; n],
    }
    let sa = a.iter().sum::<usize>();
    if sa <= k {
        let mut ans = 0;
        for i in 0..n {
            ans += a[i] * (a[i] + 1) / 2;
        }
        println!("{}", ans);
        return;
    }
    let mut ng = 1;
    let mut ok = INF;
    while ok - ng > 1 {
        let m = (ng + ok) / 2;
        let mut s = 0;
        for i in 0..n {
            if a[i] >= m {
                s += a[i] + 1 - m;
            }
        }
        if s >= k {
            ng = m;
        } else {
            ok = m;
        }
    }
    let mut ans = 0;
    let mut s = 0;
    for i in 0..n {
        if a[i] >= ok {
            ans += a[i] * (a[i] + 1) / 2 - ng * (ng + 1) / 2;
            s += a[i] + 1 - ok;
        }
    }
    ans += ng * (k - s);
    println!("{}", ans);
}
