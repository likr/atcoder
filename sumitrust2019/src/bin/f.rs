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
        t1: usize,
        t2: usize,
        mut a1: usize,
        mut a2: usize,
        mut b1: usize,
        mut b2: usize,
    }
    let mut a_a = t1 * a1 + t2 * a2;
    let mut a_b = t1 * b1 + t2 * b2;
    if a_a == a_b {
        println!("infinity");
        return;
    }
    if a_a > a_b {
        std::mem::swap(&mut a1, &mut b1);
        std::mem::swap(&mut a2, &mut b2);
        std::mem::swap(&mut a_a, &mut a_b);
    }

    if a1 < b1 {
        println!("0");
        return;
    }

    let d = t1 * (a1 - b1);
    let e = a_b - a_a;
    eprintln!("{} {}", d, e);

    if d % e == 0 {
        println!("{}", d / e * 2);
    } else {
        println!("{}", d / e * 2 + 1);
    }
}
