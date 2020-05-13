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
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        mut xl: [(isize, isize); n],
    }
    xl.sort_by_key(|&(xi, li)| (xi + li, xi));

    let mut r = -INF;

    let mut result = 0;
    for &(xi, li) in &xl {
        if r <= xi - li {
            result += 1;
            r = xi + li;
        }
    }
    println!("{}", result);
}
