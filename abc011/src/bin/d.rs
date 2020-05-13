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
        n: isize,
        d: isize,
        x: isize,
        y: isize,
    }
    if x.abs() % d != 0 || y.abs() % d != 0 {
        println!("0");
        return;
    }
    let h = x.abs() / d;
    let v = y.abs() / d;

    let mut result = 0.0;
    for p in 0..=n {
        let q = n - p;
        if p >= h && q >= v && p % 2 == h % 2 && q % 2 == v % 2 {
            let mut p1 = 1.;
            for i in 0..p {
                p1 *= (n - i) as f64;
                p1 /= (p - i) as f64;
            }
            for _ in 0..n {
                p1 /= 2.;
            }

            let mut p2 = 1.;
            let r = (p + h) / 2;
            for i in 0..r {
                p2 *= (p - i) as f64;
                p2 /= (r - i) as f64;
            }
            for _ in 0..p {
                p2 /= 2.;
            }

            let mut p3 = 1.;
            let t = (q + v) / 2;
            for i in 0..t {
                p3 *= (q - i) as f64;
                p3 /= (t - i) as f64;
            }
            for _ in 0..q {
                p3 /= 2.;
            }

            // eprintln!("{} {} {}", p1, p2, p3);
            result += p1 * p2 * p3;
        }
    }
    println!("{}", result);
}
