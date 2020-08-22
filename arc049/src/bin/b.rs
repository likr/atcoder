use ordered_float::OrderedFloat;
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
        mut xyc: [(f64, f64, f64); n],
    }
    xyc.sort_by_key(|&(x, _, _)| OrderedFloat::from(x));
    let mut l = 0.;
    let mut r = 1000000000.;
    for _ in 0..100 {
        let m = (l + r) / 2.;
        let mut ok = true;
        for i in 0..n {
            let (xi, yi, ci) = xyc[i];
            for j in 0..i {
                let (xj, yj, cj) = xyc[j];
                let x0 = xj;
                let x1 = xi;
                let (y0, y1) = if yi < yj { (yi, yj) } else { (yj, yi) };
                let c0 = cj;
                let c1 = ci;
                if x0 + m / c0 < x1 - m / c1 || y0 + m / c0 < y1 - m / c1 {
                    ok = false;
                }
            }
        }
        if ok {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", l);
}
