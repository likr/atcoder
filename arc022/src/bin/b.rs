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

fn main() {
    input! {
        n: usize,
        a:[usize; n],
    }
    let mut result = 0;
    let mut r = 0;
    let mut s = HashSet::new();
    for l in 0..n {
        while r < n && !s.contains(&a[r]) {
            s.insert(a[r]);
            r += 1;
        }
        result = max(result, r - l);
        s.remove(&a[l]);
    }
    println!("{}", result);
}
