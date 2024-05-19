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

fn f(n: usize, a: usize, x: f64, y: f64, dp: &mut HashMap<usize, f64>) -> f64 {
    if let Some(&ans) = dp.get(&n) {
        ans
    } else {
        let ans = if n == 0 {
            0.
        } else {
            let ans1 = x + f(n / a, a, x, y, dp);
            let ans2 = (6. * y + (2..=6).map(|b| f(n / b, a, x, y, dp)).sum::<f64>()) / 5.;
            if ans1 < ans2 {
                ans1
            } else {
                ans2
            }
        };
        dp.insert(n, ans);
        ans
    }
}

fn main() {
    input! {
        n: usize,
        a: usize,
        x: f64,
        y: f64,
    }
    let mut dp = HashMap::new();
    println!("{}", f(n, a, x, y, &mut dp));
}
