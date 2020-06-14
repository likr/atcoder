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
    if a[0] > 0 {
        println!("-1");
        return;
    }
    let mut result = 0usize;
    for i in 1..n {
        if a[i] > a[i - 1] + 1 {
            println!("-1");
            return;
        } else {
            result += 1;
        }
    }
    println!("{}", result);
}
