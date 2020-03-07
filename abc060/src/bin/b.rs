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
        a: isize,
        b: isize,
        c: isize,
    }
    for k in 0..b {
        let d = k * a - c;
        // eprintln!("{} {}", d, d % b);
        if d % b == 0 {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
