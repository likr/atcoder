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
        deg: usize,
        dis: usize,
    }
    let deg = deg * 10;
    let dis = (dis as f64 / 60. * 10. + 0.5).floor() as usize;
    eprintln!("{} {}", deg, dis);

    let dirs = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let mut dir = "N";
    for i in 1..dirs.len() {
        if 2250 * (i - 1) + 1125 <= deg && deg < 2250 * i + 1125 {
            dir = dirs[i];
        }
    }

    let w_min = [0, 3, 16, 34, 55, 80, 108, 139, 172, 208, 245, 285, 327];
    let mut w = 12;
    for i in 1..w_min.len() {
        if w_min[i - 1] <= dis && dis < w_min[i] {
            w = i - 1;
        }
    }

    if w == 0 {
        dir = "C";
    }

    println!("{} {}", dir, w);
}
