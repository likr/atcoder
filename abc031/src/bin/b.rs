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
        l: usize,
        h: usize,
        n: usize,
        a: [usize; n],
    }
    for &ai in &a {
        if ai < l {
            println!("{}", l - ai);
        } else if ai <= h {
            println!("0");
        } else {
            println!("-1");
        }
    }
}
