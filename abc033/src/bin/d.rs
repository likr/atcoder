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
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;
const EPS: f64 = 1e-9;

fn angle(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    ((y2 - y1) as f64).atan2((x2 - x1) as f64)
}

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    let mut right_triangles = 0;
    let mut obtuse_triangles = 0;
    for i in 0..n {
        let (xi, yi) = xy[i];

        let mut angles = (0..n)
            .filter(|&j| i != j)
            .map(|j| {
                let (xj, yj) = xy[j];
                angle(xi, yi, xj, yj)
            })
            .collect::<Vec<f64>>();
        angles.sort_by_key(|&a| OrderedFloat::from(a));

        for a in angles.clone() {
            angles.push(a + PI * 2.);
        }

        for j in 0..n - 1 {
            let k0 = angles.lower_bound_by_key(&OrderedFloat::from(PI / 2. - EPS), |&a| {
                OrderedFloat::from(a - angles[j])
            });
            let k1 = angles.upper_bound_by_key(&OrderedFloat::from(PI / 2. + EPS), |&a| {
                OrderedFloat::from(a - angles[j])
            });
            let k2 = angles.upper_bound_by_key(&OrderedFloat::from(PI + EPS), |&a| {
                OrderedFloat::from(a - angles[j])
            });
            right_triangles += k1 - k0;
            obtuse_triangles += k2 - k1;
        }
    }
    println!(
        "{} {} {}",
        n * (n - 1) * (n - 2) / 6 - right_triangles - obtuse_triangles,
        right_triangles,
        obtuse_triangles
    );
}
