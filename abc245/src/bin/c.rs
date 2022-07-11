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
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut ok = vec![vec![true; 2]; n];

    for i in 1..n {
        if (!ok[i - 1][0] || (a[i] - a[i - 1]).abs() > k)
            && (!ok[i - 1][1] || (a[i] - b[i - 1]).abs() > k)
        {
            ok[i][0] = false;
        }
        if (!ok[i - 1][0] || (b[i] - a[i - 1]).abs() > k)
            && (!ok[i - 1][1] || (b[i] - b[i - 1]).abs() > k)
        {
            ok[i][1] = false;
        }
        if !ok[i][0] && !ok[i][1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
