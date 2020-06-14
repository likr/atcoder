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
        a: usize,
        r: usize,
        n: usize,
    }
    if r == 1 {
        println!("{}", a);
        return;
    }
    let mut s = a;
    for _ in 1..n {
        s *= r;
        if s > 1000000000 {
            println!("large");
            return;
        }
    }
    println!("{}", s);
}
