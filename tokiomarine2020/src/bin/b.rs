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
        mut a: isize,
        mut v: isize,
        mut b: isize,
        mut w: isize,
        t: isize,
    }
    if b < a {
        if b - t * w >= a - t * v {
            println!("YES");
        } else {
            println!("NO");
        }
    } else {
        if b + t * w <= a + t * v {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
