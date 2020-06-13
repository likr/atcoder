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
        mut ab: [(usize, usize); n],
    }
    ab.sort_by_key(|&(ai, bi)| (ai, Reverse(bi)));
    let mut index = 0;
    let mut available_tasks = BinaryHeap::new();
    let mut score = vec![0; n + 1];
    for i in 1..=n {
        while index < n && ab[index].0 <= i {
            available_tasks.push(ab[index].1);
            index += 1;
        }
        score[i] = score[i - 1];
        if let Some(b) = available_tasks.pop() {
            score[i] += b;
        }
        println!("{}", score[i]);
    }
}
