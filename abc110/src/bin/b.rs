use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        m: usize,
        x: isize,
        y: isize,
        mut xs: [isize; n],
        mut ys: [isize; m],
    }
    xs.sort();
    ys.sort();
    let xn = *xs.last().unwrap();
    let y1 = *ys.first().unwrap();
    if xn < y1 && x < y1 && xn < y {
        println!("No War");
    } else {
        println!("War");
    }
}
