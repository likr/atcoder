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
        m: usize,
        a: [usize; m],
    }
    let mut d = vec![0; n];
    for i in 0..m {
        let k = d.lower_bound(&a[i]);
        if k == 0 {
            println!("-1");
        } else {
            println!("{}", n - k + 1);
            d[k - 1] = a[i];
        }
    }
}
