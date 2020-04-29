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
        b: isize,
        mut c: isize,
        d: isize,
    }
    let mut i = 0;
    while a > 0 && c > 0 {
        eprintln!("{} {}", a, c);
        if i % 2 == 0 {
            c -= b;
        } else {
            a -= d;
        }
        i += 1;
    }
    if a <= 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
