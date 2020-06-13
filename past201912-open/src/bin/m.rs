use ordered_float::*;
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
        m: usize,
        mut ab: [(f64, f64); n],
        mut cd: [(f64, f64); m],
    }
    let mut l = 0.;
    let mut r = 100000.;
    for _ in 0..100 {
        let m = (l + r) / 2.;
        ab.sort_by_key(|&(ai, bi)| OrderedFloat::from(bi - m * ai));
        ab.reverse();
        cd.sort_by_key(|&(ci, di)| OrderedFloat::from(di - m * ci));
        cd.reverse();
        let mut sa = (0..4).map(|i| ab[i].0).sum::<f64>();
        let mut sb = (0..4).map(|i| ab[i].1).sum::<f64>();
        if ab[4].1 - m * ab[4].0 > cd[0].1 - m * cd[0].0 {
            sa += ab[4].0;
            sb += ab[4].1;
        } else {
            sa += cd[0].0;
            sb += cd[0].1;
        }
        if sb >= m * sa {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
