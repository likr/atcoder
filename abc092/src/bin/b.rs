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
        d: usize,
        x: usize,
        a: [usize; n],
    }
    let mut s = x;
    for i in 0..n {
        let mut c = 0;
        while c * a[i] + 1 <= d {
            c += 1;
        }
        s += c;
    }
    println!("{}", s);
}
