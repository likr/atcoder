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

fn gcd(m: usize, n: usize) -> usize {
    if n > m {
        gcd(n, m)
    } else if m % n == 0 {
        n
    } else {
        gcd(n, m % n)
    }
}

fn lcm(m: usize, n: usize) -> usize {
    m * n / gcd(m, n)
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let ac = (a - 1) / c;
    let ad = (a - 1) / d;
    let acd = (a - 1) / lcm(c, d);
    let bc = b / c;
    let bd = b / d;
    let bcd = b / lcm(c, d);
    eprintln!("{} {} {}", ac, ad, acd);
    eprintln!("{} {} {}", bc, bd, bcd);
    println!("{}", b + 1 + bcd + ac + ad - a - bc - bd - acd);
}
