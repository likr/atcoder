use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
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

fn dist(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

fn disc(q1: (f64, f64), q2: (f64, f64), q3: (f64, f64)) -> (f64, f64, f64) {
    let x1 = q1.0 * q1.0 + q1.1 * q1.1;
    let x2 = q2.0 * q2.0 + q2.1 * q2.1;
    let x3 = q3.0 * q3.0 + q3.1 * q3.1;
    let a = q1.0 - q2.0;
    let b = q1.1 - q2.1;
    let c = q2.0 - q3.0;
    let d = q2.1 - q3.1;
    let cx = (d * (x1 - x2) - b * (x2 - x3)) / (2. * (a * d - b * c));
    let cy = (a * (x2 - x3) - c * (x1 - x2)) / (2. * (a * d - b * c));
    let r = dist(q1.0 - cx, q1.1 - cy);
    (cx, cy, r)
}

fn disc_with_2points(q1: (f64, f64), q2: (f64, f64)) -> (f64, f64, f64) {
    (
        (q1.0 + q2.0) / 2.,
        (q1.1 + q2.1) / 2.,
        dist(q1.0 - q2.0, q1.1 - q2.1) / 2.,
    )
}

fn includes(c: (f64, f64, f64), p: (f64, f64)) -> bool {
    let dx = c.0 - p.0;
    let dy = c.1 - p.1;
    dx * dx + dy * dy <= c.2 * c.2
}

fn mini_disc<R: Rng>(p: &mut [(f64, f64)], rng: &mut R) -> (f64, f64, f64) {
    p.shuffle(rng);
    let mut d = disc_with_2points(p[0], p[1]);
    for i in 2..p.len() {
        if !includes(d, p[i]) {
            let pi = p[i];
            d = mini_disc_with_point(&mut p[..i], pi, rng);
        }
    }
    d
}

fn mini_disc_with_point<R: Rng>(
    p: &mut [(f64, f64)],
    q: (f64, f64),
    rng: &mut R,
) -> (f64, f64, f64) {
    p.shuffle(rng);
    let mut d = disc_with_2points(q, p[0]);
    for j in 1..p.len() {
        if !includes(d, p[j]) {
            let pj = p[j];
            d = mini_disc_with_2points(&mut p[..j], pj, q);
        }
    }
    d
}

fn mini_disc_with_2points(p: &mut [(f64, f64)], q1: (f64, f64), q2: (f64, f64)) -> (f64, f64, f64) {
    let mut d = disc_with_2points(q1, q2);
    for k in 0..p.len() {
        if !includes(d, p[k]) {
            d = disc(q1, q2, p[k]);
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        mut a: [(f64, f64); n],
        mut b: [(f64, f64); n],
    }
    let mut rng = thread_rng();
    let (_, _, r1) = dbg!(mini_disc(&mut a, &mut rng));
    let (_, _, r2) = dbg!(mini_disc(&mut b, &mut rng));
    println!("{}", r2 / r1);
}
