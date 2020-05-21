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
        l: f64,
        x: f64,
        y: f64,
        s: f64,
        d: f64,
    }
    if s == d {
        println!("0");
        return;
    }
    let (d1, d2) = if d > s {
        let d1 = d - s;
        let d2 = l - d1;
        (d1, d2)
    } else {
        let d2 = s - d;
        let d1 = l - d2;
        (d1, d2)
    };
    let v1 = x + y;
    let v2 = y - x;
    if v2 < 0. {
        println!("{}", d1 / v1);
    } else {
        if d1 / v1 < d2 / v2 {
            println!("{}", d1 / v1);
        } else {
            println!("{}", d2 / v2);
        }
    }
}
