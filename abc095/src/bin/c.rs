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
        b: usize,
        c: usize,
        x: usize,
        y: usize,
    }
    if a + b <= 2 * c {
        println!("{}", x * a + y * b);
    } else {
        if x > y {
            if a <= 2 * c {
                println!("{}", a * (x - y) + 2 * c * y);
            } else {
                println!("{}", 2 * c * x);
            }
        } else if x < y {
            if b <= 2 * c {
                println!("{}", b * (y - x) + 2 * c * x);
            } else {
                println!("{}", 2 * c * y);
            }
        } else {
            println!("{}", c * (x + y));
        }
    }
}
