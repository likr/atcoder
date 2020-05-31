use itertools::Itertools;
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
        k: usize,
        a: [usize; n],
    }
    let mut b = vec![false; n - 1];
    for i in 1..n {
        b[i - 1] = a[i - 1] < a[i];
    }
    let mut result = 0usize;
    for (_, items) in b.iter().group_by(|&f| f).into_iter().filter(|(&f, _)| f) {
        let c = items.count();
        if c + 1 >= k {
            result += c + 2 - k;
        }
    }
    if k == 1 {
        println!("{}", n);
    } else {
        println!("{}", result);
    }
}
