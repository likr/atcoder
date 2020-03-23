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

    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0) += 1;
    }

    let mut s = 0usize;
    for (_, &v) in count.iter() {
        if v >= 2 {
            s += v * (v - 1) / 2;
        }
    }

    for &ak in &a {
        let c = count[&ak];
        if c >= 2 {
            println!("{}", s + (c - 1) * (c - 2) / 2 - c * (c - 1) / 2);
        } else {
            println!("{}", s - c * (c - 1) / 2);
        }
    }
}
