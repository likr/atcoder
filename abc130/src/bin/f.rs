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

fn calc_area(xyd: &Vec<(f64, f64, f64, f64)>, t: f64) -> f64 {
    let x_min = xyd
        .iter()
        .map(|item| OrderedFloat::from(item.0 + t * item.2))
        .min()
        .unwrap()
        .into_inner();
    let x_max = xyd
        .iter()
        .map(|(xi, _, dxi, _)| OrderedFloat::from(xi + t * dxi))
        .max()
        .unwrap()
        .into_inner();
    let y_min = xyd
        .iter()
        .map(|(_, yi, _, dyi)| OrderedFloat::from(yi + t * dyi))
        .min()
        .unwrap()
        .into_inner();
    let y_max = xyd
        .iter()
        .map(|(_, yi, _, dyi)| OrderedFloat::from(yi + t * dyi))
        .max()
        .unwrap()
        .into_inner();
    (x_max - x_min) * (y_max - y_min)
}

fn main() {
    input! {
        n: usize,
        xyd: [(f64, f64, char); n],
    }
    let xyd = xyd
        .into_iter()
        .map(|(xi, yi, di)| {
            let (dx, dy) = match di {
                'R' => (1., 0.),
                'L' => (-1., 0.),
                'U' => (0., 1.),
                'D' => (0., -1.),
                _ => (0., 0.),
            };
            (xi, yi, dx, dy)
        })
        .collect::<Vec<_>>();
    let mut l = 0f64;
    let mut r = 1000f64;
    for _ in 0..2000 {
        let m = (l + r) / 2.;
        if calc_area(&xyd, (l + m) / 2.) < calc_area(&xyd, (m + r) / 2.) {
            r = m;
        } else {
            l = m;
        }
    }
    eprintln!("{} {}", l, r);
    println!("{}", calc_area(&xyd, l));
}
