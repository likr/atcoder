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
        ax: isize,
        ay: isize,
        bx: isize,
        by: isize,
        n: usize,
        xy: [(isize, isize); n],
    }
    let mut count = 0;
    for i in 0..n {
        let (cx, cy) = xy[i];
        let (dx, dy) = xy[(i + 1) % n];
        let ta = (cx - dx) * (ay - cy) + (cy - dy) * (cx - ax);
        let tb = (cx - dx) * (by - cy) + (cy - dy) * (cx - bx);
        let tc = (ax - bx) * (cy - ay) + (ay - by) * (ax - cx);
        let td = (ax - bx) * (dy - ay) + (ay - by) * (ax - dx);
        if tc * td < 0 && ta * tb < 0 {
            count += 1;
        }
    }
    println!("{}", 1 + count / 2);
}
