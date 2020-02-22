use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        d: usize,
        g: usize,
        pc: [(usize, usize); d],
    }
    let mut result = INF;
    for x in 0..2usize.pow(d as u32) {
        let mut score = 0;
        let mut count = 0;
        for i in 0..d {
            let (pi, ci) = pc[i];
            let si = 100 * (i + 1);
            if x & 1 << i > 0 {
                score += si * pi + ci;
                count += pi;
            }
        }
        for i in (0..d).rev() {
            let pi = pc[i].0;
            let si = 100 * (i + 1);
            if x & 1 << i == 0 {
                if score < g {
                    if g - score > pi * si {
                        score += pi * si;
                        count += pi;
                    } else {
                        let c = (g - score + si - 1) / si;
                        score += pi * si;
                        count += c;
                    }
                }
            }
        }
        // eprintln!("{} {} {}", x, score, count);
        result = min(result, count);
    }
    println!("{}", result);
}
