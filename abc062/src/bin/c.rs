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
        h: usize,
        w: usize,
    }
    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }
    let mut result = min(h, w);
    for a in 1..=w / 2 {
        let b = h / 2;
        let mut area = [h * a, (w - a) * b, (w - a) * (h - b)];
        area.sort();
        result = min(result, area[2] - area[0]);
    }
    let (h, w) = (w, h);
    for a in 1..=w / 2 {
        let b = h / 2;
        let mut area = [h * a, (w - a) * b, (w - a) * (h - b)];
        area.sort();
        result = min(result, area[2] - area[0]);
    }
    println!("{}", result);
}
