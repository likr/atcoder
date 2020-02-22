use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        m: usize,
        mut x: [i64; n],
        mut y: [i64; m],
    }
    let x0 = x[0];
    for i in 0..n {
        x[i] -= x0;
    }
    let y0 = y[0];
    for j in 0..m {
        y[j] -= y0;
    }

    let mut d_x = vec![0; n - 1];
    d_x[0] = n - 1;
    d_x[n - 2] = d_x[0];
    let mut d = n - 1;
    for i in 1..n / 2 {
        d -= 2;
        d_x[i] = d_x[i - 1] + d;
        d_x[n - 2 - i] = d_x[i];
    }
    let mut x_total = 0;
    for i in 1..n {
        x_total += d_x[i - 1] * (x[i] - x[i - 1]) as usize;
        x_total %= M;
    }

    let mut d_y = vec![0; m - 1];
    d_y[0] = m - 1;
    d_y[m - 2] = d_y[0];
    let mut d = m - 1;
    for i in 1..m / 2 {
        d -= 2;
        d_y[i] = d_y[i - 1] + d;
        d_y[m - 2 - i] = d_y[i];
    }
    let mut y_total = 0;
    for i in 1..m {
        y_total += d_y[i - 1] * (y[i] - y[i - 1]) as usize;
        y_total %= M;
    }
    // eprintln!("{:?} {:?}", d_x, d_y);
    println!("{}", x_total * y_total % M);
}
