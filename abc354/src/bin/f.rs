use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn lis(a: &Vec<i64>) -> Vec<usize> {
    let n = a.len();
    let mut dp = vec![INF; n];
    let mut l = vec![0; n];
    for i in 0..n {
        let k = dp.lower_bound(&a[i]);
        dp[k] = a[i];
        l[i] = k + 1;
    }
    l
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut a: [i64; n],
        }
        let lis_left = lis(&a);
        a.reverse();
        for i in 0..n {
            a[i] = -a[i];
        }
        let mut lis_right = lis(&a);
        a.reverse();
        for i in 0..n {
            a[i] = -a[i];
        }
        lis_right.reverse();
        debug!(lis_left);
        debug!(lis_right);

        let m = lis_left.iter().max().unwrap();
        let mut ans = vec![];
        for i in 0..n {
            if lis_left[i] + lis_right[i] == m + 1 {
                ans.push(format!("{}", i + 1));
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.join(" "));
    }
}
