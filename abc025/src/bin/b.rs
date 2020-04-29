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
        a: isize,
        b: isize,
        sd: [(String, isize); n],
    }
    let mut x = 0isize;
    for i in 0..n {
        let (si, di) = &sd[i];
        let p = if si == "East" { 1 } else { -1 };
        let q = min(max(*di, a), b);
        x += p * q;
    }
    if x < 0 {
        println!("West {}", -x);
    } else if x > 0 {
        println!("East {}", x);
    } else {
        println!("0");
    }
}
