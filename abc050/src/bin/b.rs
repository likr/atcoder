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
        t: [usize; n],
        m: usize,
        px: [(Usize1, usize); m],
    }
    let s = t.iter().sum::<usize>();
    for &(pi, xi) in &px {
        println!("{}", s - t[pi] + xi);
    }
}
