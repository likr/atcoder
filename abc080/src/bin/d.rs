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
        _c: usize,
        mut stc: [(usize, usize, usize); n],
    }
    stc.sort_by_key(|&(si, _, ci)| (ci, si));
    let mut events = BinaryHeap::new();
    for (_, group) in &stc.iter().group_by(|&(_, _, ci)| ci) {
        let group = group.collect::<Vec<_>>();
        let m = group.len();
        let mut seq = vec![0; m];
        for i in 1..m {
            seq[i] = seq[i - 1];
            if group[i - 1].1 != group[i].0 {
                seq[i] += 1;
            }
        }
        for (_, indices) in &(0..m).group_by(|&i| seq[i]) {
            let indices = indices.collect::<Vec<_>>();
            events.push(Reverse((group[indices[0]].0, false)));
            events.push(Reverse((group[indices[indices.len() - 1]].1 + 1, true)));
        }
    }
    let mut result = 0;
    let mut count = 0;
    let mut i = 1;
    while !events.is_empty() {
        while let Some(&Reverse((j, remove))) = events.peek() {
            if i != j {
                break;
            }
            if remove {
                count -= 1;
            } else {
                count += 1;
            }
            events.pop();
        }
        result = max(result, count);
        i += 1;
    }
    println!("{}", result);
}
