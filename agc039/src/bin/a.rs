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
        s: Chars,
        k: usize,
    }
    let n = s.len();
    let count = s
        .iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(&c, group)| (c, group.count()))
        .collect::<Vec<(char, usize)>>();
    let result = if count.len() == 1 {
        n * k / 2
    } else {
        let (a, ca) = count.first().unwrap();
        let (b, cb) = count.last().unwrap();
        if a == b {
            let mut result = 0;
            for &(_, c) in &count[1..count.len() - 1] {
                result += c / 2;
            }
            k * result + ca / 2 + cb / 2 + (k - 1) * ((ca + cb) / 2)
        } else {
            let mut result = 0;
            for &(_, c) in &count {
                result += c / 2;
            }
            k * result
        }
    };
    println!("{}", result);
}
