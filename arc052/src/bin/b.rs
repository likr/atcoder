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
        q: usize,
        xrh: [(f64, f64, f64); n],
        ab: [(f64, f64); q],
    }
    for i in 0..q {
        let (ai, bi) = ab[i];
        let mut s = 0.;
        for j in 0..n {
            let (xj, rj, hj) = xrh[j];
            if bi < xj || xj + hj < ai {
                continue;
            }
            if ai <= xj && xj + hj <= bi {
                s += PI * rj * rj * hj / 3.;
            } else if ai <= xj && bi < xj + hj {
                let h2 = xj + hj - bi;
                let r2 = h2 / hj * rj;
                s += PI * (rj * rj * hj - r2 * r2 * h2) / 3.;
            } else if xj < ai && xj + hj <= bi {
                let h1 = xj + hj - ai;
                let r1 = h1 / hj * rj;
                s += PI * r1 * r1 * h1 / 3.;
            } else if xj < ai && bi < xj + hj {
                let h1 = xj + hj - ai;
                let r1 = h1 / hj * rj;
                let h2 = xj + hj - bi;
                let r2 = h2 / hj * rj;
                s += PI * (r1 * r1 * h1 - r2 * r2 * h2) / 3.;
            }
        }
        println!("{}", s);
    }
}
