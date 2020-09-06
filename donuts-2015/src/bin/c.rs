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
        h: [usize; n],
    }

    let mut x = vec![];
    for i in 0..n {
        let hi = h[i];
        println!("{}", x.len());
        while !x.is_empty() && x[x.len() - 1] <= hi {
            x.pop();
        }
        x.push(hi);
    }
}
