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
        a: [isize; n],
    }
    let mut result = -(INF as isize);
    for i in 0..n {
        let mut x = -(INF as isize);
        let mut y = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut e = 0;
            let mut o = 0;
            let c = min(i, j);
            let d = max(i, j);
            for k in c..=d {
                if (k - c) % 2 == 0 {
                    e += a[k];
                } else {
                    o += a[k];
                }
            }
            if o > x {
                x = o;
                y = e;
            }
        }
        result = max(result, y);
    }
    println!("{}", result);
}
