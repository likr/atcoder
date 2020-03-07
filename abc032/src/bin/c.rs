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
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }
    for &si in &s {
        if si == 0 {
            println!("{}", n);
            return;
        }
    }
    let k = (k as f64).ln();
    let s = s.into_iter().map(|si| (si as f64).ln()).collect::<Vec<_>>();
    let mut acc = vec![0.; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + s[i - 1];
    }
    // eprintln!("{:?}", acc);
    let mut result = 0;
    for i in 1..=n {
        let j = i + acc[i..=n].upper_bound_by(|&sj| {
            if sj - acc[i - 1] == k {
                Ordering::Equal
            } else if sj - acc[i - 1] < k + 0.000000000001 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        // eprintln!("{} {}", i, j);
        result = max(result, j - i);
    }
    println!("{}", result);
}
