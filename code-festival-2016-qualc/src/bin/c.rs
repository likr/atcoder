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
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }
    if s[n - 1] != t[0] {
        println!("0");
        return;
    }
    let top_h = t[0];
    let s_top = s.iter().position(|&si| si == top_h).unwrap();
    let t_top = t.iter().rposition(|&ti| ti == top_h).unwrap();
    if s_top > t_top {
        println!("0");
        return;
    }

    let mut fixed = vec![false; n];
    fixed[0] = true;
    for i in 1..n {
        if s[i - 1] < s[i] {
            fixed[i] = true;
        }
    }
    fixed[n - 1] = true;
    for i in (0..n - 1).rev() {
        if t[i] > t[i + 1] {
            fixed[i] = true;
        }
    }

    let mut p = 1;
    for i in 0..n {
        if fixed[i] {
            continue;
        }
        let max_h = min(s[i], t[i]);
        p = p * max_h % M;
    }
    println!("{}", p);
}
