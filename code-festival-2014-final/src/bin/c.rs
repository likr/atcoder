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
    }
    if a == 10000000000000000 {
        println!("10000");
        return;
    }
    for n in 10..10000 {
        let mut d = n;
        let mut s = 0;
        let mut b = 1;
        while d > 0 {
            s += d % 10 * b;
            b *= n;
            d /= 10;
        }
        // eprintln!("{} {}", n, s);
        if s == a {
            println!("{}", n);
            return;
        }
    }
    println!("-1");
}
