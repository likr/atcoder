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
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut states0 = vec![];
    let mut states1 = vec![];
    states0.push((0, 0));
    for i in 0..n {
        let (wi, vi) = wv[i];
        states1.clear();
        states1.push((0, 0));
        for &(wj, vj) in &states0 {
            if wj <= w {
                states1.push((wj, vj));
            }
            if wi + wj <= w {
                states1.push((wi + wj, vi + vj));
            }
        }
        states1.sort_by_key(|&(wj, vj)| (wj, -(vj as isize)));
        let mut v_max = -1;
        let removed = (0..states1.len())
            .map(|j| {
                let vj = states1[j].1 as isize;
                if vj > v_max {
                    v_max = vj;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();
        // eprintln!("{:?}", states1);
        // eprintln!("{:?}", removed);
        states0.clear();
        for j in 0..states1.len() {
            if !removed[j] {
                states0.push(states1[j]);
            }
        }
    }
    println!("{}", states0.iter().last().unwrap().1);
}
