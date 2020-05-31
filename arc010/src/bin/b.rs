use itertools::Itertools;
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
        n: usize,
        md: [String; n],
    }
    let mut days = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 1..=12 {
        days[i] += days[i - 1];
    }
    let mut holiday = vec![false; 366];
    for d in (0..366).step_by(7) {
        holiday[d] = true;
    }
    for d in (6..366).step_by(7) {
        holiday[d] = true;
    }

    for s in &md {
        let s = s.split('/').collect::<Vec<_>>();
        let m = s[0].parse::<usize>().unwrap();
        let d = s[1].parse::<usize>().unwrap();
        let mut k = days[m - 1] + d - 1;
        while k < 365 && holiday[k] {
            k += 1;
        }
        holiday[k] = true;
    }

    println!(
        "{}",
        holiday
            .iter()
            .group_by(|&f| f)
            .into_iter()
            .filter(|(&k, _)| k)
            .map(|(_, items)| items.count())
            .max()
            .unwrap()
    );
}
