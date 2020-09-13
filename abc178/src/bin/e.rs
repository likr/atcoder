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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

fn cross3(p0: Point, p1: Point, p2: Point) -> isize {
    (p2.x - p0.x) * (p1.y - p0.y) - (p1.x - p0.x) * (p2.y - p0.y)
}

fn graham_scan(points: &[Point]) -> Vec<usize> {
    let mut indices = (0..points.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| (points[i].x, points[i].y));
    indices.dedup_by_key(|i| points[*i]);

    if indices.len() <= 2 {
        return indices;
    }

    let mut upper = vec![];
    for &k in &indices {
        while upper.len() >= 2
            && cross3(
                points[upper[upper.len() - 2]],
                points[upper[upper.len() - 1]],
                points[k],
            ) <= 0
        {
            upper.pop();
        }
        upper.push(k);
    }

    let mut lower = vec![];
    for &k in &indices {
        while lower.len() >= 2
            && cross3(
                points[lower[lower.len() - 2]],
                points[lower[lower.len() - 1]],
                points[k],
            ) >= 0
        {
            lower.pop();
        }
        lower.push(k);
    }

    lower.reverse();
    upper.pop();
    lower.pop();
    for &k in &lower {
        upper.push(k);
    }
    upper
}

fn dist(i: usize, j: usize, xy: &Vec<(isize, isize)>) -> isize {
    let (x1, y1) = xy[i];
    let (x2, y2) = xy[j];
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let points = xy
        .iter()
        .map(|&(x, y)| Point { x, y })
        .collect::<Vec<Point>>();
    let ch = graham_scan(&points);

    let mut p = vec![];
    for &i in &ch {
        debug!(xy[i]);
        p.push(i);
    }
    for &i in &ch {
        p.push(i);
    }
    debug!(p);

    let mut result = 0;
    let mut l = 0;
    for k in 0..ch.len() {
        if l < k {
            l = k;
        }
        let mut d0 = dist(p[k], p[l], &xy);
        let d = loop {
            if l + 1 >= p.len() {
                break d0;
            }
            let d = dist(p[k], p[l + 1], &xy);
            // debug!(k, l + 1);
            // debug!(xy[p[k]], xy[p[l + 1]]);
            // debug!(p[k], p[l + 1], d, d0);
            if d <= d0 {
                if l > k {
                    l -= 1;
                }
                break d0;
            }
            l += 1;
            d0 = d;
        };
        // debug!(d);
        result = max(result, d);
    }
    println!("{}", result);
}
