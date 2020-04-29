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
        se: [(usize, usize); 3],
    }
    let mut result = 0;
    for i in 0..3 {
        let (si, ei) = se[i];
        result += si / 10 * ei;
    }
    println!("{}", result);
}
