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
    let n = 26;
    input! {
        d: usize,
        c: [isize; n],
        s: [[isize; n]; d],
        t: [Usize1; d],
    }
    let mut last = vec![0; n];
    let mut score = 0;
    for i in 0..d {
        let ti = t[i];
        score += s[i][ti];
        last[ti] = i + 1;
        for j in 0..n {
            score -= c[j] * (i + 1 - last[j]) as isize;
        }
        println!("{}", score);
    }
}
