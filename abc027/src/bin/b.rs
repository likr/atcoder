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
        a: [usize; n],
    }
    let s = a.iter().sum::<usize>();
    if s % n != 0 {
        println!("-1");
        return;
    }
    let t = s / n;

    let mut p = a[0];
    let mut c = 1;
    let mut result = 0;
    for i in 1..n {
        if p == t * c {
            p = a[i];
            c = 1;
        } else {
            p += a[i];
            c += 1;
            result += 1;
        }
    }
    println!("{}", result);
}
