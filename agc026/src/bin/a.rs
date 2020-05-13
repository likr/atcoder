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
        a: [usize; n],
    }
    let mut v: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        let j = v.len();
        if v.is_empty() || v[j - 1].0 != a[i] {
            v.push((a[i], 1));
        } else {
            v[j - 1].1 += 1;
        }
    }
    let mut result = 0;
    for &(_, c) in &v {
        result += c / 2;
    }
    println!("{}", result);
}
